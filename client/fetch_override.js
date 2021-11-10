// Intercepts the browser network request via XmlHttpRequest and Window.fetch.
// A callback function is stored in callback_pool where the index is and the request is passed
// as Json string to the main rust program which is then intercepted in the invoke_handler.
//
// When the rust code has finish the response it will call on `responseCallback` via eval.
// The callback_id is also passed and the response_payload.

var callback_pool = [];
var callback_pool_free = [];

(function(XHR) {
    "use strict";
    
    var stats = [];
    var timeoutId = null;
    
    var open = XHR.prototype.open;
    var send = XHR.prototype.send;
    var headers = {};
    
    XHR.prototype.open = function(method, url, async, user, pass) {
        console.log("XHR intercepting.. ", url);
        this._url = url;
        this._method = method;
        open.call(this, method, url, async, user, pass);
    };

    XHR.prototype.setRequestHeader = function(header, value) {
        console.log("setting header: "+header+" = ", value);
        headers[header] = value;
    };
    
    XHR.prototype.send = function(data) {
        console.log("data is trying to be sent..", data);
        var self = this;
        var start;
        var oldOnReadyStateChange;
        var url = this._url;
        var method = this._method;

        function onReadyCallbackFn(arg_resp) {
            open.call(self, method, url, false, null, null);
            console.log("this is called");
            Object.defineProperty(self, "readyState", {writable: true});
            Object.defineProperty(self, "status", {writable: true});
            Object.defineProperty(self, "response", {writable: true});
            Object.defineProperty(self, "responseText", {writable: true});
            self.responseText = callbackResponse.body
            self.response = callbackResponse.body
            self.readyState = 4;
            self.status = callbackResponse.code;
            var event; // this is the load event 

            if (document.createEvent) {
              event = document.createEvent("HTMLEvents");
              event.initEvent("load", true, true);
            } else {
              event = document.createEventObject();
              event.eventType = "load";
            }

            event.eventName = "load";
            //trigger the load event here
            if (document.createEvent) {
              self.dispatchEvent(event);
            } else {
              self.fireEvent("on" + event.eventType, event);
            }
        }
        
        /// register to callback pool expecting 1 argument
        let callback_id = register_to_callback_pool(onReadyCallbackFn);
        let arg = {
            method: method,
            body: data,
            url: url, 
            callback_id: callback_id,
            headers: headers
        };
        let arg_str = JSON.stringify(arg);
        window.webkit.messageHandlers.external.postMessage(arg_str);
    }
})(XMLHttpRequest);



// callback_id is needed to determine which function to call
// when there is still an executing request in action
function register_to_callback_pool(callbackFn){
        let callback_id =  callback_pool_free.pop();
        if (callback_id) {
            callback_pool[callback_id] = callbackFn;
            return callback_id;
        }else{
            let callback_id = callback_pool.push(callbackFn) - 1;
            return callback_id;
        }
}

/// called from the rust side using eval
//  data is the argument in json string
function responseCallback(callback_id, data){
    let callbackFn = callback_pool[callback_id];
    callbackFn(data);
    callback_pool_free.push(callback_id);
}


// hijack fetch function
window.fetch = function(url){
    return new Promise(function(resolve,reject){
        /// resolve is will be called after the rust code executes
        //  decode base64 then convert to Uint8Array
        function decode_wasm_b64_and_resolve(wasm_b64){
            let wasm = window.atob(wasm_b64);
            var rawLength = wasm.length;
            var array = new Uint8Array(new ArrayBuffer(rawLength));

            for(i = 0; i < rawLength; i++) {
               array[i] = wasm.charCodeAt(i);
            }
            resolve(array);
        }

        let callback_id = register_to_callback_pool(decode_wasm_b64_and_resolve);
        let arg = {
                url,
                callback_id, 
                method:"fetch"
            };
        window.webkit.messageHandlers.external.postMessage(JSON.stringify(arg));
    })
}

function dummy(arg){
    console.log("dummy function is called with arg: {}", JSON.stringify(arg));
}
