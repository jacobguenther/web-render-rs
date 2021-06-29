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
	let parts: Vec<&str> = s.split_whitespace().collect();
	let subscribers = parts[0];
	let events = parts[1];
	let notify_func = parts[2];

	format!(
		"
		if !(self.{}.is_empty() || self.{}.is_empty()) {{
			self.{}.iter().for_each(|subscriber| {{
				self.{}.iter().for_each(|event| {{
					subscriber.borrow_mut().{}(event);
				}});
			}})
		}}",
		events, subscribers, subscribers, events, notify_func
	)
	.parse()
	.unwrap()
}
#[proc_macro]
pub fn gen_notify_event_body_single(item: TokenStream) -> TokenStream {
	let lit: syn::Lit = syn::parse(item).expect("failed to parse input");
	let s = if let syn::Lit::Str(s) = lit {
		s.value()
	} else {
		panic!("expected string literal")
	};

	let parts: Vec<&str> = s.split_whitespace().collect();
	let subscribers = parts[0];
	let event = parts[1];
	let notify_func = parts[2];
	
	format!(
		"
		if let Some(ref event) = self.{} {{
			self.{}.iter().for_each(|subscriber| {{
				subscriber.borrow_mut().{}(event);
			}});
		}}", event, subscribers, notify_func).parse().unwrap()
}

#[proc_macro]
pub fn create_callback(item: TokenStream) -> TokenStream {
	let lit: syn::Lit = syn::parse(item).expect("failed to parse input");
	let s = if let syn::Lit::Str(s) = lit {
		s.value()
	} else {
		panic!("expected string literal");
	};

	let parts: Vec<&str> = s.split(" ").collect();

	let field = parts[0];
	let web_sys_event_type = parts[1];
	let event_type = parts[2];
	let element = parts[3];
	let callback_type_str = parts[4];

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
