let M=null,Q=0,K=128,V=`Object`,W=4,N=`undefined`,T=`function`,U=`string`,S=1,O=`utf-8`,J=Array,P=Error,X=Object,Y=Reflect,R=Uint8Array,L=undefined;var t=((b,c,d,e)=>{const f={a:b,b:c,cnt:S,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=Q;try{return e(c,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var i=(()=>{if(h===M||h.byteLength===Q){h=new R(a.memory.buffer)};return h});var F=((a,b)=>{});var E=(()=>{const b={};b.wbg={};b.wbg.__wbg_ok_4f114b1c058d7803=(a=>{const b=c(a).ok;return b});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===L;return b});b.wbg.__wbg_checked_fae75426dd38619c=(a=>{const b=c(a).checked;return b});b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbg_newwithstr_19bf69d1840d2816=function(){return C(((a,b)=>{const c=new Request(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_url_70f3179afe0eccd6=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_new_7d30e9d9d2deaf9d=function(){return C(((a,b)=>{const c=new URL(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_setsearch_ad0620e22e67a913=((a,b,d)=>{c(a).search=j(b,d)});b.wbg.__wbg_toString_61d1ba76c783d2bc=(a=>{const b=c(a).toString();return k(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_newwithstrandinit_9fd2fc855c6327eb=function(){return C(((a,b,d)=>{const e=new Request(j(a,b),c(d));return k(e)}),arguments)};b.wbg.__wbg_fetch_ba9c8f5d941ae5c4=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_instanceof_WorkerGlobalScope_a9d2cb51ce9a4579=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_fetch_06d656a1b748ac0d=((a,b)=>{const d=c(a).fetch(c(b));return k(d)});b.wbg.__wbg_instanceof_Response_0d25bb8436a9cefe=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_text_10c88c5e55f873c7=function(){return C((a=>{const b=c(a).text();return k(b)}),arguments)};b.wbg.__wbg_new0_c0e40662db0749ee=(()=>{const a=new Date();return k(a)});b.wbg.__wbg_getTime_af7ca51c0bcefa08=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return p(b)?Q:k(b)});b.wbg.__wbg_lastChild_8f7b6f3825115eff=(a=>{const b=c(a).lastChild;return p(b)?Q:k(b)});b.wbg.__wbg_id_bdd8815504740fa6=((b,d)=>{const e=c(d).id;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_getItem_f7e7a061bbdabefe=function(){return C(((b,d,e,f)=>{const g=c(d).getItem(j(e,f));var h=p(g)?Q:o(g,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=l;r()[b/W+ S]=i;r()[b/W+ Q]=h}),arguments)};b.wbg.__wbg_navigator_910cca0226b70083=(a=>{const b=c(a).navigator;return k(b)});b.wbg.__wbg_userAgent_4106f80b9924b065=function(){return C(((b,d)=>{const e=c(d).userAgent;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f}),arguments)};b.wbg.__wbg_setItem_2b72ddf192083111=function(){return C(((a,b,d,e,f)=>{c(a).setItem(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_new_87d841e70661f6e9=(()=>{const a=new X();return k(a)});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbg_buffer_5d1b598a01b41a42=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_d695c7957788f922=((a,b,d)=>{const e=new R(c(a),b>>>Q,d>>>Q);return k(e)});b.wbg.__wbg_of_3d7aa62bb0ab56ee=(a=>{const b=J.of(c(a));return k(b)});b.wbg.__wbg_newwithu8arraysequenceandoptions_d0ee7f095b8bf8eb=function(){return C(((a,b)=>{const d=new Blob(c(a),c(b));return k(d)}),arguments)};b.wbg.__wbg_createObjectURL_0a02ce8c75afc373=function(){return C(((b,d)=>{const e=URL.createObjectURL(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f}),arguments)};b.wbg.__wbg_sethref_e4f758ffb6abc79c=((a,b,d)=>{c(a).href=j(b,d)});b.wbg.__wbg_click_fb27a2d3b17c09c2=(a=>{c(a).click()});b.wbg.__wbg_revokeObjectURL_912e89549777c09d=function(){return C(((a,b)=>{URL.revokeObjectURL(j(a,b))}),arguments)};b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new P();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,S)}});b.wbg.__wbg_new_a979e9eedc5e81a3=function(){return C((()=>{const a=new Headers();return k(a)}),arguments)};b.wbg.__wbg_new_26bb7e688dfc365c=function(){return C((()=>{const a=new URLSearchParams();return k(a)}),arguments)};b.wbg.__wbg_localStorage_318b1c4f106a46f9=function(){return C((a=>{const b=c(a).localStorage;return p(b)?Q:k(b)}),arguments)};b.wbg.__wbg_instanceof_Error_f5ae6a28929a8190=(a=>{let b;try{b=c(a) instanceof P}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_name_90a0336d27b12317=(a=>{const b=c(a).name;return k(b)});b.wbg.__wbg_message_5dbdf59ed61bbc49=(a=>{const b=c(a).message;return k(b)});b.wbg.__wbg_toString_5326377607a05bf2=(a=>{const b=c(a).toString();return k(b)});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return p(b)?Q:k(b)});b.wbg.__wbg_get_5027b32da70f39b1=function(){return C(((a,b)=>{const d=Y.get(c(a),c(b));return k(d)}),arguments)};b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===U?e:L;var g=p(f)?Q:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/W+ S]=h;r()[b/W+ Q]=g});b.wbg.__wbg_self_086b5302bcafb962=function(){return C((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return C((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return C((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return C((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return C(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=X.is(c(a),c(b));return d});b.wbg.__wbg_set_37a50e901587b477=function(){return C(((a,b,d)=>{const e=Y.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=s(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new P(j(a,b))});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==S){b.a=Q;return !0};const c=!1;return c});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_then_5842e4e97f7beace=((a,b,d)=>{const e=c(a).then(c(b),c(d));return k(e)});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===T;return b});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return C(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return C(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===Q?L:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_instanceof_Element_f614cf57d4316979=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_0819c2800784a176=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?Q:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_target_791826e938c3e308=(a=>{const b=c(a).target;return p(b)?Q:k(b)});b.wbg.__wbg_setchecked_3b12f3d602a63e47=((a,b)=>{c(a).checked=b!==Q});b.wbg.__wbg_name_b99b509feea66304=((b,d)=>{const e=c(d).name;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_value_c93cb4b4d352228e=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_setvalue_9bd3f93b3864ddbf=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_setvalue_918a8ae77531a942=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return p(b)?Q:k(b)});b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return C((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_removeChild_14b08321b677677a=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_search_b5c7b044aaf64616=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_addEventListener_1b158e9e95e0ab00=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/W+ S]=p(d)?Q:d;r()[a/W+ Q]=!p(d)});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>Q];return k(d)});b.wbg.__wbg_bubbles_f0783dc095f8e220=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>Q});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>Q});b.wbg.__wbg_cancelBubble_191799b8e0ab3254=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/W+ S]=p(d)?Q:d;r()[a/W+ Q]=!p(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/W+ S]=p(d)?Q:d;r()[a/W+ Q]=!p(d)});b.wbg.__wbg_parentElement_86a7612dde875ba9=(a=>{const b=c(a).parentElement;return p(b)?Q:k(b)});b.wbg.__wbg_parentNode_f3957fdd408a62f7=(a=>{const b=c(a).parentNode;return p(b)?Q:k(b)});b.wbg.__wbg_instanceof_ShadowRoot_cb6366cb0956ce29=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_99e27ed8897850f2=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_setnodeValue_8656e865e9b11bbb=((a,b,d)=>{c(a).nodeValue=b===Q?L:j(b,d)});b.wbg.__wbg_value_ab23a75318ea828f=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/W+ S]=g;r()[b/W+ Q]=f});b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_from_a663e01d8dab8e44=(a=>{const b=J.from(c(a));return k(b)});b.wbg.__wbg_insertBefore_882082ef4c5d7766=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=B(b,c).slice();a.__wbindgen_free(b,c*W,W);console.error(...d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>Q});b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return C(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbindgen_closure_wrapper1781=((a,b,c)=>{const d=t(a,b,55,u);return k(d)});b.wbg.__wbindgen_closure_wrapper2709=((a,b,c)=>{const d=v(a,b,59,y);return k(d)});return b});var G=((b,c)=>{a=b.exports;I.__wbindgen_wasm_module=c;q=M;z=M;h=M;a.__wbindgen_start();return a});var p=(a=>a===L||a===M);var B=((a,b)=>{a=a>>>Q;const c=A();const d=c.subarray(a/W,a/W+ b);const e=[];for(let a=Q;a<d.length;a++){e.push(f(d[a]))};return e});var c=(a=>b[a]);var I=(async(b)=>{if(a!==L)return a;if(typeof b===N){b=new URL(`asselect_bg.wasm`,import.meta.url)};const c=E();if(typeof b===U||typeof Request===T&&b instanceof Request||typeof URL===T&&b instanceof URL){b=fetch(b)};F(c);const {instance:d,module:e}=await D(await b,c);return G(d,e)});var A=(()=>{if(z===M||z.byteLength===Q){z=new Uint32Array(a.memory.buffer)};return z});var u=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h90a0b5fde7a84c67(b,c,k(d))});var f=(a=>{const b=c(a);e(a);return b});var H=(b=>{if(a!==L)return a;const c=E();F(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return G(d,b)});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var y=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_ref__h73dbc76df5274d7a(c,d,x(e))}finally{b[w++]=L}});var s=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==M){return `${a}`};if(b==U){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==M){return `Symbol`}else{return `Symbol(${b})`}};if(b==T){const b=a.name;if(typeof b==U&&b.length>Q){return `Function(${b})`}else{return `Function`}};if(J.isArray(a)){const b=a.length;let c=`[`;if(b>Q){c+=s(a[Q])};for(let d=S;d<b;d++){c+=`, `+ s(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>S){d=c[S]}else{return toString.call(a)};if(d==V){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return V}};if(a instanceof P){return `${a.name}: ${a.message}\n${a.stack}`};return d});var x=(a=>{if(w==S)throw new P(`out of js stack`);b[--w]=a;return w});var r=(()=>{if(q===M||q.byteLength===Q){q=new Int32Array(a.memory.buffer)};return q});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:S,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=Q}}};g.original=f;return g});var D=(async(a,b)=>{if(typeof Response===T&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===T){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var o=((a,b,c)=>{if(c===L){const c=m.encode(a);const d=b(c.length,S)>>>Q;i().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,S)>>>Q;const f=i();let g=Q;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==Q){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,S)>>>Q;const b=i().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written};l=g;return e});var e=(a=>{if(a<132)return;b[a]=d;d=a});var k=(a=>{if(d===b.length)b.push(b.length+ S);const c=d;d=b[c];b[c]=a;return c});var j=((a,b)=>{a=a>>>Q;return g.decode(i().subarray(a,a+ b))});let a;const b=new J(K).fill(L);b.push(L,M,!0,!1);let d=b.length;const g=typeof TextDecoder!==N?new TextDecoder(O,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw P(`TextDecoder not available`)}};if(typeof TextDecoder!==N){g.decode()};let h=M;let l=Q;const m=typeof TextEncoder!==N?new TextEncoder(O):{encode:()=>{throw P(`TextEncoder not available`)}};const n=typeof m.encodeInto===T?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=M;let w=K;let z=M;export default I;export{H as initSync}