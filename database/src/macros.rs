/// Implement queries for a model, allowing both transactions and database pools
macro_rules! impl_queries {
    (
        for $target:ident ;

        $(
            $( #[ $meta:meta ] )*
            $visibility:vis async fn $name:ident (
                $( & $self:ident , )? $($arg_name:ident : $arg_ty:ty ),* ; $db:ident
            ) -> $result:ty
            { $( $tt:tt )* }
        )*
    ) => {
        impl $target {
            $(
                $( #[ $meta ] )*
                $visibility fn $name <'a, 'c, A>(
                    $( & $self , )?
                    $($arg_name : $arg_ty , )*
                    $db: A
                ) -> impl ::std::future::Future<Output=$result> + Send + 'a
                where
                    A: 'a + ::sqlx::Acquire<'c, Database = ::sqlx::Postgres> + Send
                {
                    async move {
                        $( $tt )*
                    }
                }
            )*
        }
    };
}
