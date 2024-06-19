use database::PgPool;
use svix::api::Svix;

macro_rules! state {
    ( $( $field:ident : $type:ty ),+ $(,)? ) => {
        /// State passed to each request handler
        #[derive(Clone)]
        pub(crate) struct AppState {
            $( pub $field: $type, )*
        }

        $(
            impl ::axum::extract::FromRef<AppState> for $type {
                fn from_ref(state: &AppState) -> Self {
                    state.$field.clone()
                }
            }
        )*
    };
}

state! {
    db: PgPool,
    schema: graphql::Schema,
}

impl AppState {
    pub(crate) fn new(db: PgPool, mail: mail::Client, svix: Svix) -> Self {
        Self {
            db: db.clone(),
            schema: graphql::schema(db, mail, svix),
        }
    }
}
