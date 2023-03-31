var searchIndex = JSON.parse('{\
"hebrides":{"doc":"Implementations for real and complex numbers.","t":[3,3,3,3,18,18,18,3,18,18,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Angle","Complex","ConversionError","DomainError","I","ONE","ONE","Real","ZERO","ZERO","abs","abs","add","add","arccos","arccos","arccosh","arccosh","arcsin","arcsin","arcsinh","arcsinh","arctan","arctan","arctanh","arctanh","azimuthal","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","ceil","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","cmp","cos","cos","cosh","cosh","div","div","eq","eq","exp","exp","floor","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from_degrees","from_radians","into","into","into","into","into","into_degrees","into_radians","is_imaginary","is_real","ln","ln","log","logf","logi","mul","mul","neg","neg","negative","new","new","norm","partial_cmp","positive","pow","powf","powi","sin","sin","sinh","sinh","sqrt","sqrt","squared","squared","sub","sub","tan","tan","tanh","tanh","to_complex","to_degrees","to_owned","to_owned","to_owned","to_owned","to_radians","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","value"],"q":["hebrides","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Representation of angular values.","Representation of complex numbers","Error type for errors involving conversion between values.","Error type for errors involving domain restrictions.","shortcut for encoding the imaginary unit as …","shortcut for describing Real::new(0.0)","shortcut for encoding 1.0 as Complex::new(Real::ONE, …","Representation of real numbers.","shortcut for describing Real::new(1.0)","shortcut for encoding 0.0 as Complex::new(Real::ZERO, …","Absolute value of <code>self</code>.","Absolute value (aka. complex norm).","","","Arccosine.","Complex arccos.","Hyperbolic arccosine.","Complex inverse hyperbolic cosine.","Arcsine.","Complex arcsin.","Hyperbolic arcsine.","Complex inverse hyperbolic sine.","Arctangent.","Complex arctan.","Hyperbolic arctangent.","Complex inverse hyperbolic tangent.","Azimuthal angle.","","","","","","","","","","","Returns the ceiling of <code>self</code>.","","","","","","","","","","Cosine.","Complex cosine.","Hyperbolic cosine.","Complex hyperbolic cosine.","","","","","e to the power of <code>self</code>.","Complex exponentiation.","Returns the floor of <code>self</code>.","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Constructs an Angle from a given value in degrees","Constructs an Angle from a given value in radians","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Converts a <code>value</code> in radians into a value in degrees","Converts a <code>value</code> in degrees into a value in radians","Returns whether or not <code>self</code> is imaginary.","Returns whether or not <code>self</code> is real.","Natural logarithm.","Complex natural logarithm.","Logarithm with base ‘base’.","Logarithm with base ‘base’.","Logarithm with base ‘base’.","","","","","Whether or not <code>self</code> is negative.","Constructs a Real from an f64.","Constructs a Complex from two f64 values","Complex norm.","","Whether or not <code>self</code> is positive.","<code>self</code> to the power of <code>power</code>.","<code>self</code> to the power of <code>power</code>.","<code>self</code> to the power of <code>power</code>.","Sine.","Complex sine.","Hyperbolic sine.","Complex hyperbolic sine.","Returns the square root of <code>self</code>.","Returns the square root of <code>self</code>.","Returns the square of <code>self</code>.","Returns <code>self</code> squared.","","","Tangent.","Complex tangent.","Hyperbolic tangent.","Complex hyperbolic tangent.","Constructs a <code>Complex</code> from <code>self</code>.","Consumes <code>self</code>, returning a value in degrees.","","","","","Consumes <code>self</code>, returning a value in radians.","","","","","","","","","","","","","","","","","","","","","Returns a copy of <code>self</code> as an f64."],"i":[0,0,0,0,2,1,2,0,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,2,3,4,6,1,2,3,4,6,1,2,1,4,6,1,2,4,6,1,2,1,1,2,1,2,1,2,1,2,1,2,1,4,4,6,6,1,1,2,2,3,4,6,1,1,1,1,1,1,1,1,1,1,1,2,3,3,3,4,6,1,2,3,3,2,2,1,2,1,1,1,1,2,1,2,1,1,2,2,1,1,1,1,1,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,3,4,6,1,2,3,4,6,1,2,3,4,6,1,1,2,3,4,6,1,2,3,4,6,1,2,1],"f":[0,0,0,0,0,0,0,0,0,0,[1,1],[2,2],[[1,1],1],[[2,2],2],[1,[[5,[3,4]]]],[2,2],[1,[[5,[3,4]]]],[2,2],[1,[[5,[3,4]]]],[2,2],[1,3],[2,2],[1,3],[2,2],[1,[[5,[3,4]]]],[2,2],[2,3],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[4,4],[6,6],[1,1],[2,2],[[]],[[]],[[]],[[]],[[1,1],7],[1,1],[2,2],[1,1],[2,2],[[1,1],1],[[2,2],2],[[1,1],8],[[2,2],8],[1,1],[2,2],[1,1],[[4,9],10],[[4,9],10],[[6,9],10],[[6,9],10],[[1,9],10],[[1,9],10],[[2,9],10],[[2,9],10],[[]],[[]],[[]],[11,1],[12,1],[13,1],[14,1],[15,1],[16,1],[17,1],[18,1],[19,1],[20,1],[[]],[[]],[18,3],[18,3],[[]],[[]],[[]],[[]],[[]],[18,18],[18,18],[2,8],[2,8],[1,[[5,[1,4]]]],[2,2],[[1,1],[[5,[1,4]]]],[[1,18],[[5,[1,4]]]],[[1,12],[[5,[1,4]]]],[[1,1],1],[[2,2],2],[1,1],[2,2],[1,8],[18,1],[[18,18],2],[2,1],[[1,1],[[21,[7]]]],[1,8],[[1,1],1],[[1,18],[[5,[1,4]]]],[[1,12],1],[1,1],[2,2],[1,1],[2,2],[1,[[5,[1,4]]]],[2,2],[1,1],[2,2],[[1,1],1],[[2,2],2],[1,1],[2,2],[1,1],[2,2],[1,2],[3,18],[[]],[[]],[[]],[[]],[3,18],[[],22],[[],22],[[],22],[[],22],[[],5],[[],5],[[],5],[2,[[5,[1,6]]]],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],5],[[],23],[[],23],[[],23],[[],23],[[],23],[1,18]],"p":[[3,"Real"],[3,"Complex"],[3,"Angle"],[3,"DomainError"],[4,"Result"],[3,"ConversionError"],[4,"Ordering"],[15,"bool"],[3,"Formatter"],[6,"Result"],[15,"i32"],[15,"i64"],[15,"u16"],[15,"f32"],[15,"u64"],[15,"u8"],[15,"i8"],[15,"f64"],[15,"u32"],[15,"i16"],[4,"Option"],[3,"String"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
