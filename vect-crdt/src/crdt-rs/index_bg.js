let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
export const SVGPathCommandType = Object.freeze({ START:0,"0":"START",LINE:1,"1":"LINE",CLOSE:2,"2":"CLOSE",BEZIER:3,"3":"BEZIER",BEZIER_REFLECT:4,"4":"BEZIER_REFLECT",BEZIER_QUAD:5,"5":"BEZIER_QUAD",BEZIER_QUAD_REFLECT:6,"6":"BEZIER_QUAD_REFLECT", });
/**
*/
export class SVGDoc {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SVGDoc.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_svgdoc_free(ptr);
    }
    /**
    * @returns {SVGDoc}
    */
    static new() {
        const ret = wasm.svgdoc_new();
        return SVGDoc.__wrap(ret);
    }
    /**
    * @param {string} group_id
    * @returns {SVGGroup | undefined}
    */
    get_group(group_id) {
        const ptr0 = passStringToWasm0(group_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgdoc_get_group(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string | undefined} group_id
    * @param {PartialSVGGroup} partial_group
    */
    add_group(group_id, partial_group) {
        var ptr0 = isLikeNone(group_id) ? 0 : passStringToWasm0(group_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_add_group(this.__wbg_ptr, ptr0, len0, addHeapObject(partial_group));
    }
    /**
    * @param {string} circle_id
    * @returns {SVGCircle | undefined}
    */
    get_circle(circle_id) {
        const ptr0 = passStringToWasm0(circle_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgdoc_get_circle(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string | undefined} group_id
    * @param {PartialSVGCircle} partial_circle
    */
    add_circle(group_id, partial_circle) {
        var ptr0 = isLikeNone(group_id) ? 0 : passStringToWasm0(group_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_add_circle(this.__wbg_ptr, ptr0, len0, addHeapObject(partial_circle));
    }
    /**
    * @param {string} circle_id
    * @param {PartialSVGCircle} edits
    */
    edit_circle(circle_id, edits) {
        const ptr0 = passStringToWasm0(circle_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_circle(this.__wbg_ptr, ptr0, len0, addHeapObject(edits));
    }
    /**
    * @param {string} rectangle_id
    * @returns {SVGRectangle | undefined}
    */
    get_rectangle(rectangle_id) {
        const ptr0 = passStringToWasm0(rectangle_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgdoc_get_rectangle(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string | undefined} group_id
    * @param {PartialSVGRectangle} partial_rectangle
    */
    add_rectangle(group_id, partial_rectangle) {
        var ptr0 = isLikeNone(group_id) ? 0 : passStringToWasm0(group_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_add_rectangle(this.__wbg_ptr, ptr0, len0, addHeapObject(partial_rectangle));
    }
    /**
    * @param {string} rectangle_id
    * @param {PartialSVGRectangle} edits
    */
    edit_rectangle(rectangle_id, edits) {
        const ptr0 = passStringToWasm0(rectangle_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_rectangle(this.__wbg_ptr, ptr0, len0, addHeapObject(edits));
    }
    /**
    * @param {string} path_id
    * @returns {SVGPath | undefined}
    */
    get_path(path_id) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgdoc_get_path(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string | undefined} group_id
    * @param {PartialSVGPath} partial_path
    */
    add_path(group_id, partial_path) {
        var ptr0 = isLikeNone(group_id) ? 0 : passStringToWasm0(group_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_add_path(this.__wbg_ptr, ptr0, len0, addHeapObject(partial_path));
    }
    /**
    * @param {string} path_id
    * @param {PartialSVGPath} partial_path
    */
    edit_path(path_id, partial_path) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_path(this.__wbg_ptr, ptr0, len0, addHeapObject(partial_path));
    }
    /**
    * @param {string} path_id
    * @param {string} point_id
    * @param {SVGPathCommandType} command_type
    */
    edit_path_point_type(path_id, point_id, command_type) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(point_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_path_point_type(this.__wbg_ptr, ptr0, len0, ptr1, len1, command_type);
    }
    /**
    * @param {string} path_id
    * @param {string} point_id
    * @param {Vec2} new_pos
    */
    edit_path_point_pos(path_id, point_id, new_pos) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(point_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_path_point_pos(this.__wbg_ptr, ptr0, len0, ptr1, len1, addHeapObject(new_pos));
    }
    /**
    * @param {string} path_id
    * @param {string} point_id
    * @param {Vec2} new_handle1
    */
    edit_path_point_handle1(path_id, point_id, new_handle1) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(point_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_path_point_handle1(this.__wbg_ptr, ptr0, len0, ptr1, len1, addHeapObject(new_handle1));
    }
    /**
    * @param {string} path_id
    * @param {string} point_id
    * @param {Vec2} new_handle2
    */
    edit_path_point_handle2(path_id, point_id, new_handle2) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(point_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.svgdoc_edit_path_point_handle2(this.__wbg_ptr, ptr0, len0, ptr1, len1, addHeapObject(new_handle2));
    }
    /**
    * @param {string} path_id
    * @param {SVGPathCommandType} command
    * @param {Vec2} pos
    */
    add_point_to_path(path_id, command, pos) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_add_point_to_path(this.__wbg_ptr, ptr0, len0, command, addHeapObject(pos));
    }
    /**
    * @param {string} object_id
    * @param {string} group_id
    * @param {number} index
    */
    move_object_to_group(object_id, group_id, index) {
        const ptr0 = passStringToWasm0(object_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(group_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.svgdoc_move_object_to_group(this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
    }
    /**
    * @param {string} object_id
    * @param {number} index
    */
    move_object_to_root(object_id, index) {
        const ptr0 = passStringToWasm0(object_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_move_object_to_root(this.__wbg_ptr, ptr0, len0, index);
    }
    /**
    * @param {string} object_id
    */
    remove_object(object_id) {
        const ptr0 = passStringToWasm0(object_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_remove_object(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @param {string} path_id
    * @param {string} point_id
    */
    remove_path_point(path_id, point_id) {
        const ptr0 = passStringToWasm0(path_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(point_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        wasm.svgdoc_remove_path_point(this.__wbg_ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * @returns {string | undefined}
    */
    save() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.svgdoc_save(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string} oplog
    */
    merge(oplog) {
        const ptr0 = passStringToWasm0(oplog, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgdoc_merge(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {SVGDocTree}
    */
    children() {
        const ret = wasm.svgdoc_children(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string}
    */
    repr() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.svgdoc_repr(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

export function __wbg_log_7283dda4f7d8a50d(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbindgen_is_undefined(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbg_crypto_d05b68a3572bb8ca(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export function __wbindgen_is_object(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbg_process_b02b3570280d0366(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
};

export function __wbg_versions_c1cb42213cedf0f5(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
};

export function __wbg_node_43b1089f407e4ec2(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
};

export function __wbindgen_is_string(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    return ret;
};

export function __wbg_msCrypto_10fc94afee92bd76(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};

export function __wbg_require_9a7e0f667ead4995() { return handleError(function () {
    const ret = module.require;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_is_function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbg_randomFillSync_b70ccbdf4926a99d() { return handleError(function (arg0, arg1) {
    getObject(arg0).randomFillSync(takeObject(arg1));
}, arguments) };

export function __wbg_getRandomValues_7e42b4fb8779dc6d() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };

export function __wbg_newnoargs_5859b6d41c6fe9f7(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_call_a79f1973a4f07d5e() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_string_get(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbg_self_086b5302bcafb962() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_window_132fa5d7546f1de5() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_globalThis_e5f801a37ad7d07b() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_global_f9a61fce4af6b7c1() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_call_f6a2bc58c19c53c6() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_now_86f7ca537c8b86d5() {
    const ret = Date.now();
    return ret;
};

export function __wbg_buffer_5d1b598a01b41a42(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export function __wbg_newwithbyteoffsetandlength_d695c7957788f922(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_new_ace717933ad7117f(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_set_74906aa30864df5a(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export function __wbg_newwithlength_728575f3bba9959b(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_subarray_7f7a652672800851(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_parse_06816e879d29d4df() { return handleError(function (arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_stringify_daa6661e90c04140() { return handleError(function (arg0) {
    const ret = JSON.stringify(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};

