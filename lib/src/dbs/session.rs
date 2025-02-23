use crate::ctx::Context;
use crate::iam::Auth;
use crate::iam::{Level, Role};
use crate::sql::value::Value;
use std::sync::Arc;

/// Specifies the current session information when processing a query.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Session {
	/// The current session [`Auth`] information
	pub au: Arc<Auth>,
	/// Whether realtime queries are supported
	pub rt: bool,
	/// The current connection IP address
	pub ip: Option<String>,
	/// The current connection origin
	pub or: Option<String>,
	/// The current connection ID
	pub id: Option<String>,
	/// The currently selected namespace
	pub ns: Option<String>,
	/// The currently selected database
	pub db: Option<String>,
	/// The currently selected authentication scope
	pub sc: Option<String>,
	/// The current scope authentication token
	pub tk: Option<Value>,
	/// The current scope authentication data
	pub sd: Option<Value>,
}

impl Session {
	/// Set the selected namespace for the session
	pub fn with_ns(mut self, ns: &str) -> Session {
		self.ns = Some(ns.to_owned());
		self
	}
	/// Set the selected database for the session
	pub fn with_db(mut self, db: &str) -> Session {
		self.db = Some(db.to_owned());
		self
	}

	// Set the realtime functionality of the session
	pub fn with_rt(mut self, rt: bool) -> Session {
		self.rt = rt;
		self
	}

	/// Retrieves the selected namespace
	pub(crate) fn ns(&self) -> Option<Arc<str>> {
		self.ns.as_deref().map(Into::into)
	}
	/// Retrieves the selected database
	pub(crate) fn db(&self) -> Option<Arc<str>> {
		self.db.as_deref().map(Into::into)
	}
	/// Checks if live queries are allowed
	pub(crate) fn live(&self) -> bool {
		self.rt
	}
	/// Convert a session into a runtime
	pub(crate) fn context<'a>(&self, mut ctx: Context<'a>) -> Context<'a> {
		// Add scope auth data
		let val: Value = self.sd.to_owned().into();
		ctx.add_value("auth", val);
		// Add scope data
		let val: Value = self.sc.to_owned().into();
		ctx.add_value("scope", val);
		// Add token data
		let val: Value = self.tk.to_owned().into();
		ctx.add_value("token", val);
		// Add session value
		let val: Value = Value::from(map! {
			"db".to_string() => self.db.to_owned().into(),
			"id".to_string() => self.id.to_owned().into(),
			"ip".to_string() => self.ip.to_owned().into(),
			"ns".to_string() => self.ns.to_owned().into(),
			"or".to_string() => self.or.to_owned().into(),
			"sc".to_string() => self.sc.to_owned().into(),
			"sd".to_string() => self.sd.to_owned().into(),
			"tk".to_string() => self.tk.to_owned().into(),
		});
		ctx.add_value("session", val);
		// Output context
		ctx
	}

	/// Create a system session for a given level and role
	pub fn for_level(level: Level, role: Role) -> Session {
		let mut sess = Session::default();

		match level {
			Level::Root => {
				sess.au = Arc::new(Auth::for_root(role));
			}
			Level::Namespace(ns) => {
				sess.au = Arc::new(Auth::for_ns(role, &ns));
				sess.ns = Some(ns);
			}
			Level::Database(ns, db) => {
				sess.au = Arc::new(Auth::for_db(role, &ns, &db));
				sess.ns = Some(ns);
				sess.db = Some(db);
			}
			_ => {}
		}
		sess
	}

	/// Create a system session for the root level with Owner role
	pub fn owner() -> Session {
		Session::for_level(Level::Root, Role::Owner)
	}

	/// Create a system session for the root level with Editor role
	pub fn editor() -> Session {
		Session::for_level(Level::Root, Role::Editor)
	}

	/// Create a system session for the root level with Viwer role
	pub fn viewer() -> Session {
		Session::for_level(Level::Root, Role::Viewer)
	}
}
