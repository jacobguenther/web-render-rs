extern crate proc_macro;
use proc_macro::TokenStream;

extern crate quote;
extern crate syn;

#[proc_macro]
pub fn gen_notify_event_body_queue(item: TokenStream) -> TokenStream {
	let lit: syn::Lit = syn::parse(item).expect("failed to parse input");
	let s = if let syn::Lit::Str(s) = lit {
		s.value()
	} else {
		panic!("expected string literal")
	};

	let mut p = s.split_whitespace();
	let subscribers = p.next().unwrap();
	let events = p.next().unwrap();
	let notify_func = p.next().unwrap();

	let mut out = String::new();
	out.push_str("if !(self.");
	out.push_str(events);
	out.push_str(".is_empty() || self.");
	out.push_str(subscribers);
	out.push_str(".is_empty()) {\n");

	out.push_str("\tself.");
	out.push_str(subscribers);
	out.push_str(".iter().for_each(|subscriber| {\n");

	out.push_str("\t\tself.");
	out.push_str(events);
	out.push_str(".iter().for_each(|event| {\n");

	out.push_str("\t\t\tsubscriber.borrow_mut().");
	out.push_str(notify_func);
	out.push_str("(event);\n");

	out.push_str("});\n})\n}\n");

	out.parse().unwrap()
}
#[proc_macro]
pub fn gen_notify_event_body_single(item: TokenStream) -> TokenStream {
	let lit: syn::Lit = syn::parse(item).expect("failed to parse input");
	let s = if let syn::Lit::Str(s) = lit {
		s.value()
	} else {
		panic!("expected string literal")
	};

	let mut p = s.split_whitespace();
	let subscribers = p.next().unwrap();
	let event = p.next().unwrap();
	let notify_func = p.next().unwrap();

	let mut out = String::new();
	out.push_str("if let Some(ref event) = self.");
	out.push_str(event);
	out.push_str("{\n");

	out.push_str("\tself.");
	out.push_str(subscribers);
	out.push_str(".iter().for_each(|subscriber| {\n");

	out.push_str("subscriber.borrow_mut().");
	out.push_str(notify_func);
	out.push_str("(event);\n");

	out.push_str("});\n}\n");

	out.parse().unwrap()
}

#[proc_macro]
pub fn create_callback(item: TokenStream) -> TokenStream {
	let lit: syn::Lit = syn::parse(item).expect("failed to parse input");
	let s = if let syn::Lit::Str(s) = lit {
		s.value()
	} else {
		panic!("expected string literal");
	};

	let mut p = s.split_whitespace();
	let field = p.next().unwrap();
	let web_sys_event_type = p.next().unwrap();
	let event_type = p.next().unwrap();
	let element = p.next().unwrap();
	let callback_type_str = p.next().unwrap();

	let mut out = String::from("{");
	out.push_str("let input_handler = input_handler.clone();");
	out.push_str("let closure = Closure::wrap(Box::new(move |event: ");
	out.push_str(&web_sys_event_type);
	out.push_str("| {");
	out.push_str("input_handler.borrow_mut().");
	out.push_str(&field);
	out.push_str(".push(");
	out.push_str(&event_type);
	out.push_str("::from(event));");
	out.push_str("}) as Box<dyn FnMut(_)>);");
	out.push_str("let _err = ");
	out.push_str(&element);
	out.push_str(".add_event_listener_with_callback(");
	out.push('"');
	out.push_str(&callback_type_str);
	out.push_str("\",");
	out.push_str("closure.as_ref().unchecked_ref(),");
	out.push_str(");");
	out.push_str("closure.forget();");
	out.push_str("}");

	out.parse().unwrap()
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
