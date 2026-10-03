var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/index.js
import { WorkerEntrypoint as ge } from "cloudflare:workers";
import G from "./3e7430be24d579346a31678f19c6d90ca79073fc-index_bg.wasm";
var B = globalThis.__worker_init_state = { criticalError: false, instanceId: 0 };
var m = class {
  static {
    __name(this, "m");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, _e.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o();
    try {
      r.__wbg_containerstartupoptions_free(e, 0);
    } catch (t) {
      s(t);
    }
  }
  get enableInternet() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_containerstartupoptions_enableInternet(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e === 16777215 ? void 0 : e !== 0;
  }
  get entrypoint() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_containerstartupoptions_entrypoint(this.__wbg_ptr);
    } catch (_) {
      s(_);
    }
    var t = ae(e[0], e[1]);
    return r.__wbindgen_free(e[0], e[1] * 4, 4), t;
  }
  get env() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_containerstartupoptions_env(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e;
  }
  set enableInternet(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_containerstartupoptions_enableInternet(this.__wbg_ptr, d(e) ? 16777215 : e ? 1 : 0);
    } catch (t) {
      s(t);
    }
  }
  set entrypoint(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let t = de(e, r.__wbindgen_malloc), _ = w;
    o();
    try {
      r.__wbg_set_containerstartupoptions_entrypoint(this.__wbg_ptr, t, _);
    } catch (i) {
      s(i);
    }
  }
  set env(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_containerstartupoptions_env(this.__wbg_ptr, e);
    } catch (t) {
      s(t);
    }
  }
};
Symbol.dispose && (m.prototype[Symbol.dispose] = m.prototype.free);
var S = class {
  static {
    __name(this, "S");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, re.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o();
    try {
      r.__wbg_intounderlyingbytesource_free(e, 0);
    } catch (t) {
      s(t);
    }
  }
  get autoAllocateChunkSize() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e >>> 0;
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw();
    o();
    try {
      r.intounderlyingbytesource_cancel(e);
    } catch (t) {
      s(t);
    }
  }
  pull(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let t;
    o();
    try {
      t = r.intounderlyingbytesource_pull(this.__wbg_ptr, e);
    } catch (_) {
      s(_);
    }
    return t;
  }
  start(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.intounderlyingbytesource_start(this.__wbg_ptr, e);
    } catch (t) {
      s(t);
    }
  }
  get type() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.intounderlyingbytesource_type(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return ne[e];
  }
};
Symbol.dispose && (S.prototype[Symbol.dispose] = S.prototype.free);
var k = class {
  static {
    __name(this, "k");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ie.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o();
    try {
      r.__wbg_intounderlyingsink_free(e, 0);
    } catch (t) {
      s(t);
    }
  }
  abort(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw(), _;
    o();
    try {
      _ = r.intounderlyingsink_abort(t, e);
    } catch (i) {
      s(i);
    }
    return _;
  }
  close() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw(), t;
    o();
    try {
      t = r.intounderlyingsink_close(e);
    } catch (_) {
      s(_);
    }
    return t;
  }
  write(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let t;
    o();
    try {
      t = r.intounderlyingsink_write(this.__wbg_ptr, e);
    } catch (_) {
      s(_);
    }
    return t;
  }
};
Symbol.dispose && (k.prototype[Symbol.dispose] = k.prototype.free);
var A = class {
  static {
    __name(this, "A");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, se.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o();
    try {
      r.__wbg_intounderlyingsource_free(e, 0);
    } catch (t) {
      s(t);
    }
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw();
    o();
    try {
      r.intounderlyingsource_cancel(e);
    } catch (t) {
      s(t);
    }
  }
  pull(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let t;
    o();
    try {
      t = r.intounderlyingsource_pull(this.__wbg_ptr, e);
    } catch (_) {
      s(_);
    }
    return t;
  }
};
Symbol.dispose && (A.prototype[Symbol.dispose] = A.prototype.free);
var v = class {
  static {
    __name(this, "v");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, oe.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o();
    try {
      r.__wbg_minifyconfig_free(e, 0);
    } catch (t) {
      s(t);
    }
  }
  get css() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_minifyconfig_css(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e !== 0;
  }
  get html() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_minifyconfig_html(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e !== 0;
  }
  get js() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_minifyconfig_js(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e !== 0;
  }
  set css(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_minifyconfig_css(this.__wbg_ptr, e);
    } catch (t) {
      s(t);
    }
  }
  set html(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_minifyconfig_html(this.__wbg_ptr, e);
    } catch (t) {
      s(t);
    }
  }
  set js(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_minifyconfig_js(this.__wbg_ptr, e);
    } catch (t) {
      s(t);
    }
  }
};
Symbol.dispose && (v.prototype[Symbol.dispose] = v.prototype.free);
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let e = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ce.unregister(this), e;
  }
  free() {
    let e = this.__destroy_into_raw();
    o();
    try {
      r.__wbg_r2range_free(e, 0);
    } catch (t) {
      s(t);
    }
  }
  get length() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_r2range_length(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e[0] === 0 ? void 0 : e[1];
  }
  get offset() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_r2range_offset(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e[0] === 0 ? void 0 : e[1];
  }
  get suffix() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    let e;
    o();
    try {
      e = r.__wbg_get_r2range_suffix(this.__wbg_ptr);
    } catch (t) {
      s(t);
    }
    return e[0] === 0 ? void 0 : e[1];
  }
  set length(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_r2range_length(this.__wbg_ptr, !d(e), d(e) ? 0 : e);
    } catch (t) {
      s(t);
    }
  }
  set offset(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_r2range_offset(this.__wbg_ptr, !d(e), d(e) ? 0 : e);
    } catch (t) {
      s(t);
    }
  }
  set suffix(e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== c) throw new Error("Invalid stale object from previous Wasm instance");
    o();
    try {
      r.__wbg_set_r2range_suffix(this.__wbg_ptr, !d(e), d(e) ? 0 : e);
    } catch (t) {
      s(t);
    }
  }
};
Symbol.dispose && (I.prototype[Symbol.dispose] = I.prototype.free);
function z() {
  c++, y = null, R = null, W = null, typeof numBytesDecoded < "u" && (numBytesDecoded = 0), typeof w < "u" && (w = 0), D = false, T = false, V = new WebAssembly.Instance(G, X()), r = V.exports, r.__wbindgen_start();
}
__name(z, "z");
function K() {
  let n;
  o();
  try {
    n = r.__worker_init_state();
  } catch (e) {
    s(e);
  }
  return n;
}
__name(K, "K");
function Q(n, e, t) {
  let _;
  o();
  try {
    _ = r.fetch(n, e, t);
  } catch (i) {
    s(i);
  }
  return _;
}
__name(Q, "Q");
function X() {
  return { __proto__: null, "./index_bg.js": { __proto__: null, __wbg_String_8564e559799eccda: /* @__PURE__ */ __name(function(e, t) {
    let _ = String(t), i = E(_, r.__wbindgen_malloc, r.__wbindgen_realloc), f = w;
    l().setInt32(e + 4, f, true), l().setInt32(e + 0, i, true);
  }, "__wbg_String_8564e559799eccda"), __wbg___wbindgen_debug_string_4687d8d8c2017d52: /* @__PURE__ */ __name(function(e, t) {
    let _ = J(t), i = E(_, r.__wbindgen_malloc, r.__wbindgen_realloc), f = w;
    l().setInt32(e + 4, f, true), l().setInt32(e + 0, i, true);
  }, "__wbg___wbindgen_debug_string_4687d8d8c2017d52"), __wbg___wbindgen_is_function_1f9d30630b8b1d3d: /* @__PURE__ */ __name(function(e) {
    return typeof e == "function";
  }, "__wbg___wbindgen_is_function_1f9d30630b8b1d3d"), __wbg___wbindgen_is_null_or_undefined_f9435d56bdcc06df: /* @__PURE__ */ __name(function(e) {
    return e == null;
  }, "__wbg___wbindgen_is_null_or_undefined_f9435d56bdcc06df"), __wbg___wbindgen_is_undefined_8865fb403f8fe9d8: /* @__PURE__ */ __name(function(e) {
    return e === void 0;
  }, "__wbg___wbindgen_is_undefined_8865fb403f8fe9d8"), __wbg___wbindgen_reinit_3f79fdb34dee3ea9: /* @__PURE__ */ __name(function() {
    T = true;
  }, "__wbg___wbindgen_reinit_3f79fdb34dee3ea9"), __wbg___wbindgen_string_get_0380ccaa2f57f0d9: /* @__PURE__ */ __name(function(e, t) {
    let _ = t, i = typeof _ == "string" ? _ : void 0;
    var f = d(i) ? 0 : E(i, r.__wbindgen_malloc, r.__wbindgen_realloc), a = w;
    l().setInt32(e + 4, a, true), l().setInt32(e + 0, f, true);
  }, "__wbg___wbindgen_string_get_0380ccaa2f57f0d9"), __wbg___wbindgen_throw_41e9ee4f547fc59a: /* @__PURE__ */ __name(function(e, t) {
    throw new WebAssembly.Exception(L, [new Error(p(e, t))]);
  }, "__wbg___wbindgen_throw_41e9ee4f547fc59a"), __wbg__wbg_cb_unref_dcc1a90847f04c41: /* @__PURE__ */ __name(function(e) {
    e._wbg_cb_unref();
  }, "__wbg__wbg_cb_unref_dcc1a90847f04c41"), __wbg_buffer_56ec2905a66f58b9: /* @__PURE__ */ __name(function(e) {
    return e.buffer;
  }, "__wbg_buffer_56ec2905a66f58b9"), __wbg_byobRequest_54f7ce41d584549a: /* @__PURE__ */ __name(function(e) {
    let t = e.byobRequest;
    return d(t) ? 0 : g(t);
  }, "__wbg_byobRequest_54f7ce41d584549a"), __wbg_byteLength_9f985b8373f1f3fe: /* @__PURE__ */ __name(function(e) {
    return e.byteLength;
  }, "__wbg_byteLength_9f985b8373f1f3fe"), __wbg_byteOffset_0bd5a6a03553639c: /* @__PURE__ */ __name(function(e) {
    return e.byteOffset;
  }, "__wbg_byteOffset_0bd5a6a03553639c"), __wbg_call_187d372bd5fdd4aa: /* @__PURE__ */ __name(function(e, t, _) {
    return e.call(t, _);
  }, "__wbg_call_187d372bd5fdd4aa"), __wbg_cause_19276d032a32d4bd: /* @__PURE__ */ __name(function(e) {
    return e.cause;
  }, "__wbg_cause_19276d032a32d4bd"), __wbg_cf_b47fe5ff6d015a0d: /* @__PURE__ */ __name(function(e) {
    let t = e.cf;
    return d(t) ? 0 : g(t);
  }, "__wbg_cf_b47fe5ff6d015a0d"), __wbg_close_185286988f30f252: /* @__PURE__ */ __name(function(e) {
    e.close();
  }, "__wbg_close_185286988f30f252"), __wbg_close_748f26698dabe269: /* @__PURE__ */ __name(function(e) {
    e.close();
  }, "__wbg_close_748f26698dabe269"), __wbg_enqueue_4a01c128f9a0de16: /* @__PURE__ */ __name(function(e, t) {
    e.enqueue(t);
  }, "__wbg_enqueue_4a01c128f9a0de16"), __wbg_error_a30e98d44009f1bb: /* @__PURE__ */ __name(function(e, t) {
    console.error(e, t);
  }, "__wbg_error_a30e98d44009f1bb"), __wbg_error_c9cf3fc2064683a9: /* @__PURE__ */ __name(function(e) {
    console.error(e);
  }, "__wbg_error_c9cf3fc2064683a9"), __wbg_get_88983e33467eae60: /* @__PURE__ */ __name(function(e, t) {
    let _ = Reflect.get(e, t);
    return d(_) ? 0 : g(_);
  }, "__wbg_get_88983e33467eae60"), __wbg_headers_6930a8bd09b630de: /* @__PURE__ */ __name(function(e) {
    return e.headers;
  }, "__wbg_headers_6930a8bd09b630de"), __wbg_instanceId_9b67ff33ae8bef8a: /* @__PURE__ */ __name(function(e) {
    return e.instanceId;
  }, "__wbg_instanceId_9b67ff33ae8bef8a"), __wbg_instanceof_Error_80a725f81f2e102d: /* @__PURE__ */ __name(function(e) {
    let t;
    try {
      t = e instanceof Error;
    } catch {
      t = false;
    }
    return t;
  }, "__wbg_instanceof_Error_80a725f81f2e102d"), __wbg_length_7f3c00c40364105e: /* @__PURE__ */ __name(function(e) {
    return e.length;
  }, "__wbg_length_7f3c00c40364105e"), __wbg_message_5f8387f0c32b90a7: /* @__PURE__ */ __name(function(e) {
    return e.message;
  }, "__wbg_message_5f8387f0c32b90a7"), __wbg_method_6d02ee0808a7de31: /* @__PURE__ */ __name(function(e, t) {
    let _ = t.method, i = E(_, r.__wbindgen_malloc, r.__wbindgen_realloc), f = w;
    l().setInt32(e + 4, f, true), l().setInt32(e + 0, i, true);
  }, "__wbg_method_6d02ee0808a7de31"), __wbg_name_e2eac7cdfa054f65: /* @__PURE__ */ __name(function(e) {
    return e.name;
  }, "__wbg_name_e2eac7cdfa054f65"), __wbg_new_343a093a3c2ffb4e: /* @__PURE__ */ __name(function(e, t) {
    return new Error(p(e, t));
  }, "__wbg_new_343a093a3c2ffb4e"), __wbg_new_617a8cdb8bb1130e: /* @__PURE__ */ __name(function() {
    return new Object();
  }, "__wbg_new_617a8cdb8bb1130e"), __wbg_new_83ca405eae412a62: /* @__PURE__ */ __name(function() {
    return new Headers();
  }, "__wbg_new_83ca405eae412a62"), __wbg_new_typed_b01cb72a8af741a3: /* @__PURE__ */ __name(function(e, t) {
    try {
      var _ = { a: e, b: t }, i = /* @__PURE__ */ __name((a, u) => {
        let b = _.a;
        _.a = 0;
        try {
          return te(b, _.b, a, u);
        } finally {
          _.a = b;
        }
      }, "i");
      return new Promise(i);
    } finally {
      _.a = 0;
    }
  }, "__wbg_new_typed_b01cb72a8af741a3"), __wbg_new_with_byte_offset_and_length_2f5d7fc2a828b74d: /* @__PURE__ */ __name(function(e, t, _) {
    return new Uint8Array(e, t >>> 0, _ >>> 0);
  }, "__wbg_new_with_byte_offset_and_length_2f5d7fc2a828b74d"), __wbg_new_with_length_3da0ad195f6f63ba: /* @__PURE__ */ __name(function(e) {
    return new Uint8Array(e >>> 0);
  }, "__wbg_new_with_length_3da0ad195f6f63ba"), __wbg_new_with_opt_buffer_source_and_init_c1b859a23c80afca: /* @__PURE__ */ __name(function(e, t) {
    return new Response(e, t);
  }, "__wbg_new_with_opt_buffer_source_and_init_c1b859a23c80afca"), __wbg_new_with_opt_readable_stream_and_init_df114f2d364f216d: /* @__PURE__ */ __name(function(e, t) {
    return new Response(e, t);
  }, "__wbg_new_with_opt_readable_stream_and_init_df114f2d364f216d"), __wbg_new_with_opt_str_and_init_33bef1b272950372: /* @__PURE__ */ __name(function(e, t, _) {
    return new Response(e === 0 ? void 0 : p(e, t), _);
  }, "__wbg_new_with_opt_str_and_init_33bef1b272950372"), __wbg_queueMicrotask_9833f9a49df95a49: /* @__PURE__ */ __name(function(e) {
    return e.queueMicrotask;
  }, "__wbg_queueMicrotask_9833f9a49df95a49"), __wbg_queueMicrotask_a72f977e97f23c5f: /* @__PURE__ */ __name(function(e) {
    queueMicrotask(e);
  }, "__wbg_queueMicrotask_a72f977e97f23c5f"), __wbg_resolve_0076e10020304ede: /* @__PURE__ */ __name(function(e) {
    return Promise.resolve(e);
  }, "__wbg_resolve_0076e10020304ede"), __wbg_respond_c102fabcc79e5ef0: /* @__PURE__ */ __name(function(e, t) {
    e.respond(t >>> 0);
  }, "__wbg_respond_c102fabcc79e5ef0"), __wbg_set_145a351398b48c65: /* @__PURE__ */ __name(function(e, t, _) {
    return Reflect.set(e, t, _);
  }, "__wbg_set_145a351398b48c65"), __wbg_set_575d3ddb70fe831d: /* @__PURE__ */ __name(function(e, t, _) {
    e.set(fe(t, _));
  }, "__wbg_set_575d3ddb70fe831d"), __wbg_set_af5932e3f78b5648: /* @__PURE__ */ __name(function(e, t, _, i, f) {
    e.set(p(t, _), p(i, f));
  }, "__wbg_set_af5932e3f78b5648"), __wbg_set_criticalError_3904d60fe2a0b05b: /* @__PURE__ */ __name(function(e, t) {
    e.criticalError = t !== 0;
  }, "__wbg_set_criticalError_3904d60fe2a0b05b"), __wbg_set_headers_69ef7c46a2e6c742: /* @__PURE__ */ __name(function(e, t) {
    e.headers = t;
  }, "__wbg_set_headers_69ef7c46a2e6c742"), __wbg_set_instanceId_9ad5caac0a4e519c: /* @__PURE__ */ __name(function(e, t) {
    e.instanceId = t >>> 0;
  }, "__wbg_set_instanceId_9ad5caac0a4e519c"), __wbg_set_status_678f122cbe435d95: /* @__PURE__ */ __name(function(e, t) {
    e.status = t;
  }, "__wbg_set_status_678f122cbe435d95"), __wbg_static_accessor_GLOBAL_266715b9d96ba635: /* @__PURE__ */ __name(function() {
    let e = typeof global > "u" ? null : global;
    return d(e) ? 0 : g(e);
  }, "__wbg_static_accessor_GLOBAL_266715b9d96ba635"), __wbg_static_accessor_GLOBAL_THIS_10fb7dc1ae063179: /* @__PURE__ */ __name(function() {
    let e = typeof globalThis > "u" ? null : globalThis;
    return d(e) ? 0 : g(e);
  }, "__wbg_static_accessor_GLOBAL_THIS_10fb7dc1ae063179"), __wbg_static_accessor_INIT_STATE_e3fce2ad8a08e94e: /* @__PURE__ */ __name(function() {
    return B;
  }, "__wbg_static_accessor_INIT_STATE_e3fce2ad8a08e94e"), __wbg_static_accessor_SELF_0b583911f537483a: /* @__PURE__ */ __name(function() {
    let e = typeof self > "u" ? null : self;
    return d(e) ? 0 : g(e);
  }, "__wbg_static_accessor_SELF_0b583911f537483a"), __wbg_static_accessor_WINDOW_d7f903d1508cbdc4: /* @__PURE__ */ __name(function() {
    let e = typeof window > "u" ? null : window;
    return d(e) ? 0 : g(e);
  }, "__wbg_static_accessor_WINDOW_d7f903d1508cbdc4"), __wbg_then_e71170d78fcf8954: /* @__PURE__ */ __name(function(e, t) {
    return e.then(t);
  }, "__wbg_then_e71170d78fcf8954"), __wbg_url_7bfada5c2297bc70: /* @__PURE__ */ __name(function(e, t) {
    let _ = t.url, i = E(_, r.__wbindgen_malloc, r.__wbindgen_realloc), f = w;
    l().setInt32(e + 4, f, true), l().setInt32(e + 0, i, true);
  }, "__wbg_url_7bfada5c2297bc70"), __wbg_view_9c570f33e8d6ab96: /* @__PURE__ */ __name(function(e) {
    let t = e.view;
    return d(t) ? 0 : g(t);
  }, "__wbg_view_9c570f33e8d6ab96"), __wbindgen_generic_0000000000000001: /* @__PURE__ */ __name(function(e, t) {
    return ue(e, t, ee);
  }, "__wbindgen_generic_0000000000000001"), __wbindgen_generic_0000000000000002: /* @__PURE__ */ __name(function(e, t) {
    return p(e, t);
  }, "__wbindgen_generic_0000000000000002"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(function() {
    let e = r.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }, "__wbindgen_init_externref_table"), __wbindgen_jstag: WebAssembly.JSTag, __wbindgen_rethrow_critical: /* @__PURE__ */ __name(function(e) {
    throw new Error("Critical error", { cause: e });
  }, "__wbindgen_rethrow_critical") } };
}
__name(X, "X");
var L = new WebAssembly.Tag({ parameters: ["externref"] });
var U;
var D = false;
function Y() {
  D = true;
  try {
    let n = q()[r.__abort_handler.value / 4];
    n && r.__wbindgen_export.get(n)();
  } catch {
  }
}
__name(Y, "Y");
function s(n) {
  throw n instanceof WebAssembly.Exception && n.is(L) ? n.getArg(L, 0) : (q()[U] = 1, Y(), n);
}
__name(s, "s");
function o() {
  if (U ??= r.__instance_terminated.value / 4, q()[U]) {
    if (D || Y(), T) {
      z();
      return;
    }
    throw new Error("Module terminated");
  } else T && z();
}
__name(o, "o");
function ee(n, e, t) {
  let _;
  o();
  try {
    _ = r.wasm_bindgen_fdee7feede9107ce___convert__closures_____invoke___wasm_bindgen_fdee7feede9107ce___JsValue__core_608f92abc48d28da___result__Result_____wasm_bindgen_fdee7feede9107ce___JsError___true_(n, e, t);
  } catch (i) {
    s(i);
  }
  if (_[1]) throw be(_[0]);
}
__name(ee, "ee");
function te(n, e, t, _) {
  o();
  try {
    r.wasm_bindgen_fdee7feede9107ce___convert__closures_____invoke___js_sys_a2c2ee530679bcf2___Function_fn_wasm_bindgen_fdee7feede9107ce___JsValue_____wasm_bindgen_fdee7feede9107ce___sys__Undefined___js_sys_a2c2ee530679bcf2___Function_fn_wasm_bindgen_fdee7feede9107ce___JsValue_____wasm_bindgen_fdee7feede9107ce___sys__Undefined_______true_(n, e, t, _);
  } catch (i) {
    s(i);
  }
}
__name(te, "te");
var ne = ["bytes"];
var c = 0;
var _e = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: e }) => {
  e === c && r.__wbg_containerstartupoptions_free(n, 1);
});
var re = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: e }) => {
  e === c && r.__wbg_intounderlyingbytesource_free(n, 1);
});
var ie = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: e }) => {
  e === c && r.__wbg_intounderlyingsink_free(n, 1);
});
var se = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: e }) => {
  e === c && r.__wbg_intounderlyingsource_free(n, 1);
});
var oe = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: e }) => {
  e === c && r.__wbg_minifyconfig_free(n, 1);
});
var ce = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: n, instance: e }) => {
  e === c && r.__wbg_r2range_free(n, 1);
});
function g(n) {
  let e = r.__externref_table_alloc();
  return r.__wbindgen_externrefs.set(e, n), e;
}
__name(g, "g");
var H = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((n) => {
  n.instance === c && r.__wbindgen_destroy_closure(n.a, n.b);
});
function J(n) {
  let e = typeof n;
  if (e == "number" || e == "boolean" || n == null) return `${n}`;
  if (e == "string") return `"${n}"`;
  if (e == "symbol") {
    let i = n.description;
    return i == null ? "Symbol" : `Symbol(${i})`;
  }
  if (e == "function") {
    let i = n.name;
    return typeof i == "string" && i.length > 0 ? `Function(${i})` : "Function";
  }
  if (Array.isArray(n)) {
    let i = n.length, f = "[";
    i > 0 && (f += J(n[0]));
    for (let a = 1; a < i; a++) f += ", " + J(n[a]);
    return f += "]", f;
  }
  let t = /\[object ([^\]]+)\]/.exec(toString.call(n)), _;
  if (t && t.length > 1) _ = t[1];
  else return toString.call(n);
  if (_ == "Object") try {
    return "Object(" + JSON.stringify(n) + ")";
  } catch {
    return "Object";
  }
  return n instanceof Error ? `${n.name}: ${n.message}
${n.stack}` : _;
}
__name(J, "J");
function ae(n, e) {
  n = n >>> 0;
  let t = l(), _ = [];
  for (let i = n; i < n + 4 * e; i += 4) _.push(r.__wbindgen_externrefs.get(t.getUint32(i, true)));
  return r.__externref_drop_slice(n, e), _;
}
__name(ae, "ae");
function fe(n, e) {
  return n = n >>> 0, j().subarray(n / 1, n / 1 + e);
}
__name(fe, "fe");
var y = null;
function l() {
  return (y === null || y.buffer.detached === true || y.buffer.detached === void 0 && y.buffer !== r.memory.buffer) && (y = new DataView(r.memory.buffer)), y;
}
__name(l, "l");
var R = null;
function q() {
  return (R === null || R.byteLength === 0) && (R = new Int32Array(r.memory.buffer)), R;
}
__name(q, "q");
function p(n, e) {
  return le(n >>> 0, e);
}
__name(p, "p");
var W = null;
function j() {
  return (W === null || W.byteLength === 0) && (W = new Uint8Array(r.memory.buffer)), W;
}
__name(j, "j");
function d(n) {
  return n == null;
}
__name(d, "d");
function ue(n, e, t) {
  let _ = { a: n, b: e, cnt: 1, instance: c }, i = /* @__PURE__ */ __name((...f) => {
    if (_.instance !== c) throw new Error("Cannot invoke closure from previous WASM instance");
    _.cnt++;
    let a = _.a;
    _.a = 0;
    try {
      return t(a, _.b, ...f);
    } finally {
      _.a = a, i._wbg_cb_unref();
    }
  }, "i");
  return i._wbg_cb_unref = () => {
    --_.cnt === 0 && (r.__wbindgen_destroy_closure(_.a, _.b), _.a = 0, H.unregister(_));
  }, H.register(i, _, _), i;
}
__name(ue, "ue");
function de(n, e) {
  let t = e(n.length * 4, 4) >>> 0;
  for (let _ = 0; _ < n.length; _++) {
    let i = g(n[_]);
    l().setUint32(t + 4 * _, i, true);
  }
  return w = n.length, t;
}
__name(de, "de");
function E(n, e, t) {
  if (t === void 0) {
    let u = F.encode(n), b = e(u.length, 1) >>> 0;
    return j().subarray(b, b + u.length).set(u), w = u.length, b;
  }
  let _ = n.length, i = e(_, 1) >>> 0, f = j(), a = 0;
  for (; a < _; a++) {
    let u = n.charCodeAt(a);
    if (u > 127) break;
    f[i + a] = u;
  }
  if (a !== _) {
    a !== 0 && (n = n.slice(a)), i = t(i, _, _ = a + n.length * 3, 1) >>> 0;
    let u = j().subarray(i + a, i + _), b = F.encodeInto(n, u);
    a += b.written, i = t(i, _, a, 1) >>> 0;
  }
  return w = a, i;
}
__name(E, "E");
var T = false;
function be(n) {
  let e = r.__wbindgen_externrefs.get(n);
  return r.__externref_table_dealloc(n), e;
}
__name(be, "be");
var Z = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
Z.decode();
function le(n, e) {
  return Z.decode(j().subarray(n, n + e));
}
__name(le, "le");
var F = new TextEncoder();
"encodeInto" in F || (F.encodeInto = function(n, e) {
  let t = F.encode(n);
  return e.set(t), { read: n.length, written: t.length };
});
var w = 0;
var V = new WebAssembly.Instance(G, X());
var r = V.exports;
r.__wbindgen_start();
Error.stackTraceLimit = 100;
var h = K();
function $() {
  h.criticalError && (console.log("Reinitializing Wasm application"), z(), h.criticalError = false, h.instanceId++);
}
__name($, "$");
addEventListener("error", (n) => {
  N(n.error);
});
function N(n) {
  n instanceof WebAssembly.RuntimeError && (console.error("Critical", n), h.criticalError = true);
}
__name(N, "N");
var O = class extends ge {
  static {
    __name(this, "O");
  }
};
O.prototype.fetch = function(e) {
  return Q.call(this, e, this.env, this.ctx);
};
var he = { set: /* @__PURE__ */ __name((n, e, t, _) => Reflect.set(n.instance, e, t, _), "set"), has: /* @__PURE__ */ __name((n, e) => Reflect.has(n.instance, e), "has"), deleteProperty: /* @__PURE__ */ __name((n, e) => Reflect.deleteProperty(n.instance, e), "deleteProperty"), apply: /* @__PURE__ */ __name((n, e, t) => Reflect.apply(n.instance, e, t), "apply"), construct: /* @__PURE__ */ __name((n, e, t) => Reflect.construct(n.instance, e, t), "construct"), getPrototypeOf: /* @__PURE__ */ __name((n) => Reflect.getPrototypeOf(n.instance), "getPrototypeOf"), setPrototypeOf: /* @__PURE__ */ __name((n, e) => Reflect.setPrototypeOf(n.instance, e), "setPrototypeOf"), isExtensible: /* @__PURE__ */ __name((n) => Reflect.isExtensible(n.instance), "isExtensible"), preventExtensions: /* @__PURE__ */ __name((n) => Reflect.preventExtensions(n.instance), "preventExtensions"), getOwnPropertyDescriptor: /* @__PURE__ */ __name((n, e) => Reflect.getOwnPropertyDescriptor(n.instance, e), "getOwnPropertyDescriptor"), defineProperty: /* @__PURE__ */ __name((n, e, t) => Reflect.defineProperty(n.instance, e, t), "defineProperty"), ownKeys: /* @__PURE__ */ __name((n) => Reflect.ownKeys(n.instance), "ownKeys") };
var P = { construct(n, e, t) {
  try {
    $();
    let _ = { instance: Reflect.construct(n, e, t), instanceId: h.instanceId, ctor: n, args: e, newTarget: t };
    return new Proxy(_, { ...he, get(i, f, a) {
      i.instanceId !== h.instanceId && (i.instance = Reflect.construct(i.ctor, i.args, i.newTarget), i.instanceId = h.instanceId);
      let u = Reflect.get(i.instance, f, a);
      return typeof u != "function" ? u : u.constructor === Function ? new Proxy(u, { apply(b, M, C) {
        $();
        try {
          return b.apply(M, C);
        } catch (x) {
          throw N(x), x;
        }
      } }) : new Proxy(u, { async apply(b, M, C) {
        $();
        try {
          return await b.apply(M, C);
        } catch (x) {
          throw N(x), x;
        }
      } });
    } });
  } catch (_) {
    throw h.criticalError = true, _;
  }
} };
var Ie = new Proxy(O, P);
var xe = new Proxy(m, P);
var Ee = new Proxy(v, P);
var Re = new Proxy(I, P);

// ../../Users/Mishr/AppData/Local/npm-cache/_npx/32026684e21afda6/node_modules/wrangler/templates/middleware/middleware-ensure-req-body-drained.ts
var drainBody = /* @__PURE__ */ __name(async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } finally {
    try {
      if (request.body !== null && !request.bodyUsed) {
        const reader = request.body.getReader();
        while (!(await reader.read()).done) {
        }
      }
    } catch (e) {
      console.error("Failed to drain the unused request body.", e);
    }
  }
}, "drainBody");
var middleware_ensure_req_body_drained_default = drainBody;

// ../../Users/Mishr/AppData/Local/npm-cache/_npx/32026684e21afda6/node_modules/wrangler/templates/middleware/middleware-miniflare3-json-error.ts
function reduceError(e) {
  return {
    name: e?.name,
    message: e?.message ?? String(e),
    stack: e?.stack,
    cause: e?.cause === void 0 ? void 0 : reduceError(e.cause)
  };
}
__name(reduceError, "reduceError");
var jsonError = /* @__PURE__ */ __name(async (request, env, _ctx, middlewareCtx) => {
  try {
    return await middlewareCtx.next(request, env);
  } catch (e) {
    const error = reduceError(e);
    const body = JSON.stringify(error);
    const headers = {
      "Content-Type": "application/json",
      "MF-Experimental-Error-Stack": "true"
    };
    const encoded = encodeURIComponent(body);
    if (encoded.length <= 8192) {
      headers["MF-Experimental-Error-Stack-Payload"] = encoded;
    }
    return new Response(body, { status: 500, headers });
  }
}, "jsonError");
var middleware_miniflare3_json_error_default = jsonError;

// .wrangler/tmp/bundle-d5QQpz/middleware-insertion-facade.js
var __INTERNAL_WRANGLER_MIDDLEWARE__ = [
  middleware_ensure_req_body_drained_default,
  middleware_miniflare3_json_error_default
];
var middleware_insertion_facade_default = Ie;

// ../../Users/Mishr/AppData/Local/npm-cache/_npx/32026684e21afda6/node_modules/wrangler/templates/middleware/common.ts
var __facade_middleware__ = [];
function __facade_register__(...args) {
  __facade_middleware__.push(...args.flat());
}
__name(__facade_register__, "__facade_register__");
function __facade_invokeChain__(request, env, ctx, dispatch, middlewareChain) {
  const [head, ...tail] = middlewareChain;
  const middlewareCtx = {
    dispatch,
    next(newRequest, newEnv) {
      return __facade_invokeChain__(newRequest, newEnv, ctx, dispatch, tail);
    }
  };
  return head(request, env, ctx, middlewareCtx);
}
__name(__facade_invokeChain__, "__facade_invokeChain__");
function __facade_invoke__(request, env, ctx, dispatch, finalMiddleware) {
  return __facade_invokeChain__(request, env, ctx, dispatch, [
    ...__facade_middleware__,
    finalMiddleware
  ]);
}
__name(__facade_invoke__, "__facade_invoke__");

// .wrangler/tmp/bundle-d5QQpz/middleware-loader.entry.ts
var __Facade_ScheduledController__ = class ___Facade_ScheduledController__ {
  constructor(scheduledTime, cron, noRetry) {
    this.scheduledTime = scheduledTime;
    this.cron = cron;
    this.#noRetry = noRetry;
  }
  scheduledTime;
  cron;
  static {
    __name(this, "__Facade_ScheduledController__");
  }
  #noRetry;
  noRetry() {
    if (!(this instanceof ___Facade_ScheduledController__)) {
      throw new TypeError("Illegal invocation");
    }
    this.#noRetry();
  }
};
function wrapExportedHandler(worker) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return worker;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  const fetchDispatcher = /* @__PURE__ */ __name(function(request, env, ctx) {
    if (worker.fetch === void 0) {
      throw new Error("Handler does not export a fetch() function.");
    }
    return worker.fetch(request, env, ctx);
  }, "fetchDispatcher");
  return {
    ...worker,
    fetch(request, env, ctx) {
      const dispatcher = /* @__PURE__ */ __name(function(type, init) {
        if (type === "scheduled" && worker.scheduled !== void 0) {
          const controller = new __Facade_ScheduledController__(
            Date.now(),
            init.cron ?? "",
            () => {
            }
          );
          return worker.scheduled(controller, env, ctx);
        }
      }, "dispatcher");
      return __facade_invoke__(request, env, ctx, dispatcher, fetchDispatcher);
    }
  };
}
__name(wrapExportedHandler, "wrapExportedHandler");
function wrapWorkerEntrypoint(klass) {
  if (__INTERNAL_WRANGLER_MIDDLEWARE__ === void 0 || __INTERNAL_WRANGLER_MIDDLEWARE__.length === 0) {
    return klass;
  }
  for (const middleware of __INTERNAL_WRANGLER_MIDDLEWARE__) {
    __facade_register__(middleware);
  }
  return class extends klass {
    #fetchDispatcher = /* @__PURE__ */ __name((request, env, ctx) => {
      this.env = env;
      this.ctx = ctx;
      if (super.fetch === void 0) {
        throw new Error("Entrypoint class does not define a fetch() function.");
      }
      return super.fetch(request);
    }, "#fetchDispatcher");
    #dispatcher = /* @__PURE__ */ __name((type, init) => {
      if (type === "scheduled" && super.scheduled !== void 0) {
        const controller = new __Facade_ScheduledController__(
          Date.now(),
          init.cron ?? "",
          () => {
          }
        );
        return super.scheduled(controller);
      }
    }, "#dispatcher");
    fetch(request) {
      return __facade_invoke__(
        request,
        this.env,
        this.ctx,
        this.#dispatcher,
        this.#fetchDispatcher
      );
    }
  };
}
__name(wrapWorkerEntrypoint, "wrapWorkerEntrypoint");
var WRAPPED_ENTRY;
if (typeof middleware_insertion_facade_default === "object") {
  WRAPPED_ENTRY = wrapExportedHandler(middleware_insertion_facade_default);
} else if (typeof middleware_insertion_facade_default === "function") {
  WRAPPED_ENTRY = wrapWorkerEntrypoint(middleware_insertion_facade_default);
}
var middleware_loader_entry_default = WRAPPED_ENTRY;
export {
  xe as ContainerStartupOptions,
  Ee as MinifyConfig,
  Re as R2Range,
  __INTERNAL_WRANGLER_MIDDLEWARE__,
  middleware_loader_entry_default as default
};
//# sourceMappingURL=index.js.map
