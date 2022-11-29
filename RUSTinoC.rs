use std::env;
use std::num::ParseIntError;


fn get_version() -> String {
   return "1.0 29/11/2022".to_string();
}

fn help() {

	println!("\n *******   **     **  ******** ********** **                     ****** ");
	println!("/**////** /**    /** **////// /////**/// //                     **////**");
	println!("/**   /** /**    /**/**           /**     ** *******   ******  **    // ");
	println!("/*******  /**    /**/*********    /**    /**//**///** **////**/**       ");
	println!("/**///**  /**    /**////////**    /**    /** /**  /**/**   /**/**       ");
	println!("/**  //** /**    /**       /**    /**    /** /**  /**/**   /**//**    **");
	println!("/**   //**//*******  ********     /**    /** ***  /**//******  //****** ");
	println!("//     //  ///////  ////////      //     // ///   //  //////    ////// \n");
	println!("RUSTinoC version {}\n",get_version());
    	println!("Use: RUSTinoC <Type Conv> <Value To Conv>");
	println!("[...] are optional parameters\n");
	println!("<Type Conv>     : Conversion Type to apply, values permitted are : \n\n");
	println!("                  -hd : Hex to Decimal");
	println!("                  -dh : Decimal to Hex");
	println!("                  -bd : Binary to Decimal");
	println!("                  -db : Decimal to Binary");
	println!("                  -od : Octal to Decimal");
	println!("                  -do : Decimal to Octal");
	println!("<Value To Conv> : Value to Convert\n");
	println!("Examples : \n");
	println!("\tRUSTinoC -hd a0ef\n");
	println!("\t\t\tHex 'a0ef' ---> Dec '-24337'\n");
	println!("\tRUSTinoC -dh 250\n");
	println!("\t\t\tDec '250' ---> Hex 'fa'\n\n");

}

fn print_dec(value_in: String, value_out: Result<i64, ParseIntError>, type_in: String, type_out: String) {
	match value_out {
            	Ok(value) => println!("\n{type_in} '{}' ---> {type_out} '{:?}'\n", value_in, value),
            	Err(error) => println!("\n{type_in} '{}' ---> '{:?}'\n", value_in, error),
        }
}

fn print_gen(value_in: String, value_out: Result<i64, ParseIntError>, type_in: String, type_out: String) {
	match value_out {
        	Ok(value) if type_out == "Hex" => println!("\n{type_in} '{}' ---> {type_out} '{:x}' \n", value_in, value),
        	Ok(value) if type_out == "Bin" => println!("\n{type_in} '{}' ---> {type_out} '{:b}' \n", value_in, value),
        	Ok(value) if type_out == "Oct" => println!("\n{type_in} '{}' ---> {type_out} '{:o}' \n", value_in, value),
	        Err(error) => println!("\n{type_in} '{}' ---> '{:?}'\n", value_in, error),
		Ok(_) => println!("\n INTERNAL ERROR. \n "),
	}
}

// Hexadecimal tp Decimal
fn hd(hex_value: String) {
	let _dec_value = i64::from_str_radix(hex_value.as_str(), 16);
	print_dec(hex_value, _dec_value, "Hex".to_string(), "Dec".to_string() );
}

// Octal to Decimal
fn od(hex_value: String) {
	let _dec_value = i64::from_str_radix(hex_value.as_str(), 8);
	print_dec(hex_value, _dec_value, "Oct".to_string(), "Dec".to_string() );
}

// Binaty to Decimal
fn bd(hex_value: String) {
	let _dec_value = i64::from_str_radix(hex_value.as_str(), 2);
	print_dec(hex_value, _dec_value, "Bin".to_string(), "Dec".to_string() );
}

fn dh(dec_value: String) {
	let _dec_value = i64::from_str_radix(dec_value.as_str(), 10);
	print_gen(dec_value, _dec_value, "Dec".to_string(), "Hex".to_string());
}

fn db(dec_value: String) {
	let _dec_value = i64::from_str_radix(dec_value.as_str(), 10);
	print_gen(dec_value, _dec_value, "Dec".to_string(), "Bin".to_string());
}

fn _do(dec_value: String) {
	let _dec_value = i64::from_str_radix(dec_value.as_str(), 10);
	print_gen(dec_value, _dec_value, "Dec".to_string(), "Oct".to_string());
}


fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
	  help();	
	} else {
	  match args[1].as_str() {
		"-hd" => hd(args[2].to_string()),
		"-dh" => dh(args[2].to_string()),
		"-od" => od(args[2].to_string()),
		"-do" => _do(args[2].to_string()),
		"-bd" => bd(args[2].to_string()),
		"-db" => db(args[2].to_string()),
		_=> println!("ERROR Option {} not managed!!!!!", args[1]),
	  }
	}
}