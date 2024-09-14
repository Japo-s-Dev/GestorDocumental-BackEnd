// region:    --- Modules

mod error;

pub use self::error::{Error, Result};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: i64,
}

// Constructor.
impl Ctx {
	pub fn root_ctx() -> Self {
		Ctx { user_id: 0 }
	}

	pub fn new(user_id: i64) -> Result<Self> {
		if user_id == 0 {
			Err(Error::CtxCannotNewRootCtx)
		} else {
			Ok(Self { user_id })
		}
	}
}

// Property Accessors.
impl Ctx {
	pub fn user_id(&self) -> i64 {
		self.user_id
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_ctx() {
        let ctx = Ctx::root_ctx();
        assert_eq!(ctx.user_id(), 0, "El user_id del contexto raíz debe ser 0");
    }

    #[test]
    fn test_new_ctx_valid_user_id() {
        let user_id = 42;
        let ctx = Ctx::new(user_id).expect("Debe poder crear un Ctx con un user_id válido");
        assert_eq!(ctx.user_id(), user_id, "El user_id debe coincidir con el proporcionado");
    }

    #[test]
    fn test_new_ctx_invalid_user_id() {
        let result = Ctx::new(0);
        assert!(result.is_err(), "No se debe permitir crear un Ctx con user_id 0");
        if let Err(e) = result {
            match e {
                Error::CtxCannotNewRootCtx => (),
                _ => panic!("El error debe ser CtxCannotNewRootCtx"),
            }
        }
    }

    #[test]
    fn test_user_id_accessor() {
        let user_id = 100;
        let ctx = Ctx::new(user_id).unwrap();
        assert_eq!(ctx.user_id(), user_id, "El método user_id debe devolver el user_id correcto");
    }
}

