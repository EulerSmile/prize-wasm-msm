use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array};
use wasm_bindgen::JsValue;

#[link(name = "mcore")]
extern {
    fn ECP_muln_rust(p: *mut u8, n: i32, x: *const u8, ix: *mut i32, e: *const u8, ie: *mut i32, ib: *mut i32);
}

#[wasm_bindgen]
pub fn compute_msm(points: Array, scalars: Array) -> Array {
    let size = std::cmp::min(points.length(), scalars.length());

    let mut x = Vec::new();
    let mut e = Vec::new();

    points.iter().into_iter().for_each(|pt: JsValue|{
        let pt = Array::from(&pt);
        let mut xs: Vec<u8> = Uint8Array::new(&pt.get(0)).to_vec();
        // Arkworks use Little-Endian, so we need reverse x and y.
        xs.reverse();
        let mut ys = Uint8Array::new(&pt.get(1)).to_vec();
        ys.reverse();
        let _inf = pt.get(2);

        x.push(0x04);
        x.extend_from_slice(&xs);
        x.extend_from_slice(&ys);
    });

    scalars.iter().into_iter().for_each(|sc: JsValue| {
        let sc: Vec<u8> = Uint8Array::new(&sc).to_vec();
        e.extend_from_slice(&sc);
    });

    let mut out = Vec::with_capacity(97);
    let mut cx: Vec<i32> = Vec::with_capacity((size * 45) as usize);
    let mut ce: Vec<i32> = Vec::with_capacity((size * 14) as usize);
    let mut cb: Vec<i32> = Vec::with_capacity(8192 * 45);
    
    unsafe {
        ECP_muln_rust(out.as_mut_ptr(), size as i32, x.as_ptr(), cx.as_mut_ptr(), e.as_ptr(), ce.as_mut_ptr(), cb.as_mut_ptr());
        out.set_len(97);
    };

    // Again, we need re-serialize Miracl's bytes.
    out.remove(0); // remove the first 0x4.
    out[0..48].reverse();
    out[48..96].reverse();

    let point = Array::new_with_length(3);
    point.set(0, Uint8Array::from(&out[0..48]).into());
    point.set(1, Uint8Array::from(&out[48..96]).into());
    point.set(2, false.into());

    point
}