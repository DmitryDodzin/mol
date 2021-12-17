#[macro_export]
macro_rules! impl_events_unwrapper {
  (
     $(#[$meta:meta])*
     $vis:vis enum $name:ident {
        $(
        $(#[$variant_meta:meta])*
        $variant_name:ident($variant_type:ident)
        ),*$(,)+
    }
  ) => {
    $(#[$meta])*
      $vis enum $name{
        $(
        $(#[$variant_meta:meta])*
        $variant_name(Box<$variant_type>),
        )*
      }

      impl $name {
        pub fn name(&self) -> $crate::WebhookEvents {
          match self {
            $(
                $name::$variant_name(_) => $crate::WebhookEvents::$variant_name,
            )*
          }
        }
      }

      #[allow(clippy::redundant_closure, clippy::large_enum_variant)]
      impl<'a> std::convert::TryInto<$name> for ($crate::WebhookEvents, &'a [u8]) {
        type Error = serde_json::Error;
        fn try_into(self) -> Result<$name, Self::Error> {
          match self.0 {
            $(
                $crate::WebhookEvents::$variant_name => {
                  serde_json::from_slice::<$crate::$variant_type>(self.1)
                    .map(|event| $crate::Events::$variant_name(Box::new(event)))
                }
            )*
            _ => unimplemented!(),
          }
        }
      }
  };
}

#[cfg(test)]
mod tests {

  #[macro_export]
  macro_rules! test_from_sample {
    ($name:ident, $event:tt, $source:expr) => {
      #[test]
      fn $name() {
        let raw = std::fs::read_to_string($source).expect("test case not found");

        let event = serde_json::from_str::<$event>(&raw);

        if let Err(ref error) = event {
          println!("{:#?}", error);
        }

        assert!(event.is_ok());
      }
    };
  }
}
