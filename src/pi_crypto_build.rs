use pi_vm::bonmgr::{BonMgr, StructMeta, FnMeta, jstype_ptr,ptr_jstype, CallResult};
use pi_vm::adapter::{JSType, JS};
use std::sync::Arc;
use std::mem::transmute;
use pi_math;
use std::ops::Drop;
use pi_crypto;



fn call_266558349(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in exchange";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



	let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.to_bytes();



    let result = pi_crypto::ed25519::exchange(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,526967798);


    Some(CallResult::Ok)
}


fn call_2282179587(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in keypair";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_crypto::ed25519::keypair(jst0);
	let array = js.new_array();
    let result_elem = result.0;
    let ptr = Box::into_raw(Box::new(result_elem)) as usize;let result_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,2521161042);

js.set_index(&array, 0, &result_elem);
    let result_elem = result.1;
    let ptr = Box::into_raw(Box::new(result_elem)) as usize;let result_elem = ptr_jstype(js.get_objs(), js.clone(), ptr,526967798);

js.set_index(&array, 1, &result_elem);

    Some(CallResult::Ok)
}


fn call_1005885597(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in sign";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



	let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.to_bytes();



    let result = pi_crypto::ed25519::sign(jst0,jst1);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,2521161042);


    Some(CallResult::Ok)
}


fn call_1115867356(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in verify";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



	let jst1 = &v[1];
	if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst1 = jst1.to_bytes();



	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    let result = pi_crypto::ed25519::verify(jst0,jst1,jst2);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1476345609(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in ripemd160";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_crypto::hash::ripemd160(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,3995272273);


    Some(CallResult::Ok)
}


fn call_2108893530(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in keccak256";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_crypto::hash::keccak256(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,526967798);


    Some(CallResult::Ok)
}


fn call_842379557(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in dhash160";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_crypto::hash::dhash160(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,3995272273);


    Some(CallResult::Ok)
}


fn call_1125159944(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in dhash256";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_crypto::hash::dhash256(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,526967798);


    Some(CallResult::Ok)
}


fn call_796485226(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in siphash24";

	let jst0 = &v[0];
    if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst0.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst0 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


	let jst1 = &v[1];
    if !jst1.is_uint8_array() && !jst1.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let arr = unsafe{*(jst1.to_bytes().as_ptr() as usize as *const [u8; 8])};
    let jst1 = unsafe {
        transmute::<[u8; 8], u64>(arr)
    }; 


	let jst2 = &v[2];
	if !jst2.is_uint8_array() && !jst2.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst2 = jst2.to_bytes();



    let result = pi_crypto::hash::siphash24(jst0,jst1,jst2);let result = js.new_u64(result);

    Some(CallResult::Ok)
}


fn call_235181891(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in checksum";

	let jst0 = &v[0];
	if !jst0.is_uint8_array() && !jst0.is_array_buffer(){return Some(CallResult::Err(String::from(param_error))); }
    let jst0 = jst0.to_bytes();



    let result = pi_crypto::hash::checksum(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,3974239134);


    Some(CallResult::Ok)
}


fn call_1252421489(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_crypto::bls::BlsIdVec::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,1875205449);


    Some(CallResult::Ok)
}


fn call_2592527877(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_crypto::bls::BlsSecKeyVec::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,2934268916);


    Some(CallResult::Ok)
}


fn call_3404883075(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_crypto::bls::BlsPubKeyVec::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,3840517932);


    Some(CallResult::Ok)
}


fn call_2903230657(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in new";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_crypto::bls::BlsSigVec::new(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,4060246115);


    Some(CallResult::Ok)
}


fn call_2498464569(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_init";

	let jst0 = &v[0];
    if !jst0.is_number(){return Some(CallResult::Err(String::from(param_error)));}
    let jst0 = match jst0.get_u32(){
        0 => pi_crypto::bls::Curve::MclBnCurveFp254BNb,
        1 => pi_crypto::bls::Curve::MclBnCurveFp382_1,
        2 => pi_crypto::bls::Curve::MclBnCurveFp382_2,
        3 => pi_crypto::bls::Curve::MclBnCurveFp462,
        4 => pi_crypto::bls::Curve::MclBnCurveSNARK1,
        5 => pi_crypto::bls::Curve::MclBls12CurveFp381,
        _ => panic!("enum type error")
    };


    let result = pi_crypto::bls::bls_init(jst0);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1295262082(js: Arc<JS>) -> Option<CallResult>{

    let result = pi_crypto::bls::bls_get_op_unit_size();let result = js.new_u32(result as u32);

    Some(CallResult::Ok)
}


fn call_2496411899(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_get_curve_order";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_crypto::bls::bls_get_curve_order(jst0);
    match result{
        Some(v) => { let v = js.new_str(v);
    
 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_755737870(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_get_field_order";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


    let result = pi_crypto::bls::bls_get_field_order(jst0);
    match result{
        Some(v) => { let v = js.new_str(v);
    
 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3253072797(js: Arc<JS>) -> Option<CallResult>{

    let result = pi_crypto::bls::bls_get_generator_of_g2();
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,1617625763);


    Some(CallResult::Ok)
}


fn call_4280890483(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_set_int";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_i32();


    let result = pi_crypto::bls::bls_id_set_int(jst0);
    let ptr = Box::into_raw(Box::new(result)) as usize;let result = ptr_jstype(js.get_objs(), js.clone(), ptr,3094164306);


    Some(CallResult::Ok)
}


fn call_2402380511(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_set_dec_str";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_str();


    let result = pi_crypto::bls::bls_id_set_dec_str(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3094164306);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_2426850537(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_set_hex_str";

	let jst0 = &v[0];
	if !jst0.is_string(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_str();


    let result = pi_crypto::bls::bls_id_set_hex_str(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3094164306);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3075954650(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_get_dec_str";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3094164306, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    let result = pi_crypto::bls::bls_id_get_dec_str(jst0,jst1);
    match result{
        Some(v) => { let v = js.new_str(v);
    
 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3801863647(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_get_hex_str";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3094164306, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    let result = pi_crypto::bls::bls_id_get_hex_str(jst0,jst1);
    match result{
        Some(v) => { let v = js.new_str(v);
    
 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1719604587(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_hash_to_secret_key";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    let result = pi_crypto::bls::bls_hash_to_secret_key(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,187111440);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3025531400(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_get_public_key";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 187111440, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


    let result = pi_crypto::bls::bls_get_public_key(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,1617625763);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3723291352(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_get_pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 187111440, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


    let result = pi_crypto::bls::bls_get_pop(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3966088300);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1669774542(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_verify_pop";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3966088300, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSignature) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1617625763, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


    let result = pi_crypto::bls::bls_verify_pop(jst0,jst1);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1235807017(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_serialize";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3094164306, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    let result = pi_crypto::bls::bls_id_serialize(jst0,jst1);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3671848448(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_secret_key_serialize";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 187111440, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


    let result = pi_crypto::bls::bls_secret_key_serialize(jst0,jst1);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1900424700(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_public_key_serialize";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1617625763, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


    let result = pi_crypto::bls::bls_public_key_serialize(jst0,jst1);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_2045530324(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_signature_serialize";

	let jst0 = &v[0];
	if !jst0.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst0 = jst0.get_u32() as usize;


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3966088300, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSignature) };


    let result = pi_crypto::bls::bls_signature_serialize(jst0,jst1);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,104530634);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_298607248(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_deserialize";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    let result = pi_crypto::bls::bls_id_deserialize(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3094164306);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_2029782143(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_secret_key_deserialize";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    let result = pi_crypto::bls::bls_secret_key_deserialize(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,187111440);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1922268706(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_public_key_deserialize";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    let result = pi_crypto::bls::bls_public_key_deserialize(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,1617625763);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_760927771(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_signature_deserialize";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 104530634, true, param_error).expect("");
	let jst0 = *unsafe { Box::from_raw(ptr as *mut Vec<u8>) };


    let result = pi_crypto::bls::bls_signature_deserialize(jst0);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3966088300);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_1304117942(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_id_is_equal";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3094164306, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3094164306, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    let result = pi_crypto::bls::bls_id_is_equal(jst0,jst1);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1202562609(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_secret_key_is_equal";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 187111440, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 187111440, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


    let result = pi_crypto::bls::bls_secret_key_is_equal(jst0,jst1);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1494397139(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_public_key_is_equal";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1617625763, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1617625763, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


    let result = pi_crypto::bls::bls_public_key_is_equal(jst0,jst1);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_1251457612(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_signature_is_equal";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3966088300, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSignature) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3966088300, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSignature) };


    let result = pi_crypto::bls::bls_signature_is_equal(jst0,jst1);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}


fn call_3750445483(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_secret_key_share";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 187111440, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3094164306, false, param_error).expect("");
	let jst2 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    let result = pi_crypto::bls::bls_secret_key_share(jst0,jst1,jst2);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,187111440);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3551222567(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_public_key_share";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1617625763, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


	let jst1 = &v[1];
	if !jst1.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst1 = jst1.get_u32() as usize;


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 3094164306, false, param_error).expect("");
	let jst2 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    let result = pi_crypto::bls::bls_public_key_share(jst0,jst1,jst2);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,1617625763);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3778283533(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_add_id_to_vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 1875205449, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut pi_crypto::bls::BlsIdVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3094164306, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsId) };


    pi_crypto::bls::bls_add_id_to_vec(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_2172313629(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_add_secret_key_to_vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2934268916, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut pi_crypto::bls::BlsSecKeyVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 187111440, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


    pi_crypto::bls::bls_add_secret_key_to_vec(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_3718730423(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_add_public_key_to_vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3840517932, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut pi_crypto::bls::BlsPubKeyVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1617625763, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


    pi_crypto::bls::bls_add_public_key_to_vec(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_263952757(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_add_signature_to_vec";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4060246115, false, param_error).expect("");
	let jst0 = unsafe { &mut *(ptr as *mut pi_crypto::bls::BlsSigVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 3966088300, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSignature) };


    pi_crypto::bls::bls_add_signature_to_vec(jst0,jst1);
    Some(CallResult::Ok)
}


fn call_1087017908(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_secret_key_recover";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 2934268916, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecKeyVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1875205449, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsIdVec) };


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;


    let result = pi_crypto::bls::bls_secret_key_recover(jst0,jst1,jst2);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,187111440);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_993477813(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_public_key_recover";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3840517932, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPubKeyVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1875205449, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsIdVec) };


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;


    let result = pi_crypto::bls::bls_public_key_recover(jst0,jst1,jst2);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,1617625763);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3587763353(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_signature_recover";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 4060246115, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSigVec) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1875205449, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsIdVec) };


	let jst2 = &v[2];
	if !jst2.is_number(){ return Some(CallResult::Err(String::from(param_error)));}
	let jst2 = jst2.get_u32() as usize;


    let result = pi_crypto::bls::bls_signature_recover(jst0,jst1,jst2);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3966088300);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_3188209906(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_sign";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 187111440, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSecretKey) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 2886438122, true, param_error).expect("");
	let jst1 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    let result = pi_crypto::bls::bls_sign(jst0,jst1);
    match result{
        Some(v) => { 
    let ptr = Box::into_raw(Box::new(v)) as usize;let v = ptr_jstype(js.get_objs(), js.clone(), ptr,3966088300);

 }
        None => { let result = js.new_undefined(); }
    };

    Some(CallResult::Ok)
}


fn call_2084703123(js: Arc<JS>, v:Vec<JSType>) -> Option<CallResult>{
	let param_error = "param error in bls_verify";

	let jst0 = &v[0];
    let ptr = jstype_ptr(&jst0, js.clone(), 3966088300, false, param_error).expect("");
	let jst0 = unsafe { &*(ptr as *const pi_crypto::bls::BlsSignature) };


	let jst1 = &v[1];
    let ptr = jstype_ptr(&jst1, js.clone(), 1617625763, false, param_error).expect("");
	let jst1 = unsafe { &*(ptr as *const pi_crypto::bls::BlsPublicKey) };


	let jst2 = &v[2];
    let ptr = jstype_ptr(&jst2, js.clone(), 2886438122, true, param_error).expect("");
	let jst2 = *unsafe { Box::from_raw(ptr as *mut Arc<Vec<u8>>)}.clone();


    let result = pi_crypto::bls::bls_verify(jst0,jst1,jst2);let result = js.new_boolean(result);

    Some(CallResult::Ok)
}
pub fn register(mgr: &BonMgr){
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H256")}, 526967798);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H512")}, 2521161042);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H160")}, 3995272273);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_math::hash::H32")}, 3974239134);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsIdVec")}, 1875205449);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsSecKeyVec")}, 2934268916);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsPubKeyVec")}, 3840517932);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsSigVec")}, 4060246115);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::Curve")}, 2254569071);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsPublicKey")}, 1617625763);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsId")}, 3094164306);
    mgr.regist_struct_meta(StructMeta{name:String::from("Vec<u8>")}, 104530634);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsSecretKey")}, 187111440);
    mgr.regist_struct_meta(StructMeta{name:String::from("pi_crypto::bls::BlsSignature")}, 3966088300);
    mgr.regist_struct_meta(StructMeta{name:String::from("Arc<Vec<u8>>")}, 2886438122);
    mgr.regist_fun_meta(FnMeta::CallArg(call_266558349), 266558349);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2282179587), 2282179587);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1005885597), 1005885597);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1115867356), 1115867356);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1476345609), 1476345609);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2108893530), 2108893530);
    mgr.regist_fun_meta(FnMeta::CallArg(call_842379557), 842379557);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1125159944), 1125159944);
    mgr.regist_fun_meta(FnMeta::CallArg(call_796485226), 796485226);
    mgr.regist_fun_meta(FnMeta::CallArg(call_235181891), 235181891);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1252421489), 1252421489);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2592527877), 2592527877);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3404883075), 3404883075);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2903230657), 2903230657);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2498464569), 2498464569);
    mgr.regist_fun_meta(FnMeta::Call(call_1295262082), 1295262082);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2496411899), 2496411899);
    mgr.regist_fun_meta(FnMeta::CallArg(call_755737870), 755737870);
    mgr.regist_fun_meta(FnMeta::Call(call_3253072797), 3253072797);
    mgr.regist_fun_meta(FnMeta::CallArg(call_4280890483), 4280890483);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2402380511), 2402380511);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2426850537), 2426850537);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3075954650), 3075954650);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3801863647), 3801863647);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1719604587), 1719604587);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3025531400), 3025531400);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3723291352), 3723291352);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1669774542), 1669774542);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1235807017), 1235807017);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3671848448), 3671848448);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1900424700), 1900424700);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2045530324), 2045530324);
    mgr.regist_fun_meta(FnMeta::CallArg(call_298607248), 298607248);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2029782143), 2029782143);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1922268706), 1922268706);
    mgr.regist_fun_meta(FnMeta::CallArg(call_760927771), 760927771);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1304117942), 1304117942);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1202562609), 1202562609);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1494397139), 1494397139);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1251457612), 1251457612);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3750445483), 3750445483);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3551222567), 3551222567);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3778283533), 3778283533);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2172313629), 2172313629);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3718730423), 3718730423);
    mgr.regist_fun_meta(FnMeta::CallArg(call_263952757), 263952757);
    mgr.regist_fun_meta(FnMeta::CallArg(call_1087017908), 1087017908);
    mgr.regist_fun_meta(FnMeta::CallArg(call_993477813), 993477813);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3587763353), 3587763353);
    mgr.regist_fun_meta(FnMeta::CallArg(call_3188209906), 3188209906);
    mgr.regist_fun_meta(FnMeta::CallArg(call_2084703123), 2084703123);
}