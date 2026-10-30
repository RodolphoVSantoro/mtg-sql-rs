fn add_str_equal_filter<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    equal_filter: &str,
    equal_filter_value: &str,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    builder.add_condition();
    builder
        .builder
        .push(format!("{} = ", equal_filter))
        .push_bind(format!("{}", equal_filter_value));

    count_builder.add_condition();
    count_builder
        .builder
        .push(format!("{} = ", equal_filter))
        .push_bind(format!("{}", equal_filter_value));
    return (builder, count_builder);
}

fn add_str_equal_filters<'a>(
    mut builder: SqlQueryBuilder<'a>,
    mut count_builder: SqlQueryBuilder<'a>,
    filter: &CardFilter,
) -> (SqlQueryBuilder<'a>, SqlQueryBuilder<'a>) {
    for equal_filter in STR_EQUAL_FILTERS {
        if let Some(equal_filter_value) = filter
            .get(equal_filter)
            .and_then(|v| v.as_str())
            .map(|s| s as &str)
        {
            (builder, count_builder) =
                add_str_equal_filter(builder, count_builder, equal_filter, equal_filter_value);
        };
        if let Some(equal_filter_values) =
            filter
                .get(equal_filter)
                .and_then(|v| v.as_array())
                .map(|s| {
                    s.into_iter()
                        .map(|v| v.as_i64())
                        .filter(|v| v.is_some())
                        .map(|v| v.unwrap())
                        .collect::<Vec<i64>>()
                })
        {
            builder.begin_multi_filter();
            count_builder.begin_multi_filter();
            for equal_filter_value in equal_filter_values {
                builder.add_multi_filter();
                count_builder.add_multi_filter();
                (builder, count_builder) =
                    add_num_equal_filter(builder, count_builder, equal_filter, equal_filter_value);
            }
            builder.end_multi_filter();
            count_builder.end_multi_filter();
        };
    }

    return (builder, count_builder);
}