extern crate phrases;

fn	main()	{
	let	plus_two	=	|x|	{
			 		let	mut	result:	i32	=	x;
					result	+=	1;
					result	+=	1;
					result
	};

		println!("x is {}",plus_two(2) );
}
