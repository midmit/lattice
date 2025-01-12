use crate::{
    l_map, obj,
    types::{CallerCtx, LMap, Object, Value},
};

// __lat_dbg(&arg1, &arg2, ...)
pub fn __lat_dbg(ctx: &mut CallerCtx) {
    // println!("\n\x1b[33m__lat_dbg\x1b[0m({:?})\n", &ctx.args());
    let args = ctx.args();

    for arg in args {
        print!("\x1b[1m\x1b[31m[DEBUG]\x1b[0m {:?}\n", &arg);
    }
}

// __lat_newmap() -> map
pub fn __lat_newmap(ctx: &mut CallerCtx) {
    // println!("\n\x1b[33m__lat_newmap\x1b[0m({:?})\n", &ctx.args());
    ctx.set_return_t(obj!(l_map!(LMap::new())));
    // println!(
    //     "\n\x1b[33m__lat_newmap\x1b[0m returning with context: {:?}\n",
    //     &ctx
    // );
}

// lat_mapset(&map, key, value)
pub fn __lat_mapset(ctx: &mut CallerCtx) {
    // println!("\n\x1b[33m__lat_mapset\x1b[0m({:?})\n", &ctx.args());

    let args = ctx.args();

    let v = args.pop().expect("Value argument is missing");
    let k = args.pop().expect("Key argument is missing");
    let mut map = args.pop().expect("Map argument is missing");

    let map_mut = Object::make_mut(&mut map);

    if let Value::Map(map) = map_mut {
        if let Value::String(k) = k.as_ref() {
            map.set(k.clone(), v.clone());
        } else {
            ctx.set_error("Key is not of type string");
        }
    } else {
        ctx.set_error("First argument must be of type map");
    }

    ctx.push_arg(map);
    ctx.push_arg(k);
    ctx.push_arg(v);

    // println!(
    //     "\n\x1b[33m__lat_mapset\x1b[0m returning with context: {:?}\n",
    //     &ctx
    // );
}

// __lat_mapget(&map, &key) -> &value
// pub fn __lat_mapget(ctx: &mut CallerCtx) {
//     let args = ctx.args();
//     let mut map = args.remove(0);
//     let k = args.remove(0);
//
//     if let Value::Map(ref mut map) = map {
//         if let Value::String(ref k) = k {
//             ctx.set_return_t(map.get(k));
//         } else {
//             ctx.set_error("Key argument must be of type String");
//         }
//     } else {
//         ctx.set_error("First argument must be of type Map");
//     }
//
//     ctx.push_arg(map);
//     ctx.push_arg(k);
// }
//
// // __lat_forget(arg1, arg2, ...)
// pub fn __lat_forget(ctx: &mut CallerCtx) {
//     let args = ctx.args();
//
//     for i in 0..args.len() {
//         args[i] = Value::Null;
//     }
// }
