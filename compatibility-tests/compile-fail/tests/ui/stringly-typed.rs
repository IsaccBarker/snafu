mod and_context {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(context, other)]
    struct Error;
}

mod with_arguments {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(other(true))]
    struct Error;
}

mod missing_message {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(other)]
    struct Error;
}

mod double_message {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(other)]
    struct Error {
        message: String,
        message: String,
    }
}

mod with_context_fields {
    use snafu::Snafu;

    #[derive(Debug, Snafu)]
    #[snafu(other)]
    struct Error {
        message: String,
        user_id: i32,
    }
}

fn main() {}
