#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    serde :: Deserialize,
    serde ::
Serialize,
)]
pub struct __________InternalFiltrationVideoFilters {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub user_id: Option<i32>,
    pub channel_id: Option<i32>,
    pub url: Option<Option<String>>,
    pub language: Option<String>,
    pub stage: Option<VideoStage>,
    pub error: Option<bool>,
    pub original_url: Option<String>,
    pub original_duration: Option<Option<String>>,
    pub start_time: Option<String>,
    pub end_time: Option<Option<String>>,
    pub tags: Option<Option<String>>,
    pub created_at: Option<NaiveDateTime>,
    pub created_at_start: Option<NaiveDateTime>,
    pub created_at_end: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_at_start: Option<NaiveDateTime>,
    pub updated_at_end: Option<NaiveDateTime>,
    pub deleted_at: Option<Option<NaiveDateTime>>,
    pub deleted_at_start: Option<Option<NaiveDateTime>>,
    pub deleted_at_end: Option<Option<NaiveDateTime>>,
    pub uploaded_at: Option<Option<NaiveDateTime>>,
    pub uploaded_at_start: Option<Option<NaiveDateTime>>,
    pub uploaded_at_end: Option<Option<NaiveDateTime>>,
}
impl crate::database::queries::filter::FilterableOptions for InternalFiltrationVideoFilters {
    fn filter_fields(&self) -> Vec<&'static str> {
        let mut fields: Vec<&'static str> = vec![];
        if self.id.is_some() {
            fields.push("id");
        }
        if self.title.is_some() {
            fields.push("title");
        }
        if self.description.is_some() {
            fields.push("description");
        }
        if self.user_id.is_some() {
            fields.push("user_id");
        }
        if self.channel_id.is_some() {
            fields.push("channel_id");
        }
        if self.url.is_some() {
            fields.push("url");
        }
        if self.language.is_some() {
            fields.push("language");
        }
        if self.stage.is_some() {
            fields.push("stage");
        }
        if self.error.is_some() {
            fields.push("error");
        }
        if self.original_url.is_some() {
            fields.push("original_url");
        }
        if self.original_duration.is_some() {
            fields.push("original_duration");
        }
        if self.start_time.is_some() {
            fields.push("start_time");
        }
        if self.end_time.is_some() {
            fields.push("end_time");
        }
        if self.tags.is_some() {
            fields.push("tags");
        }
        if self.created_at.is_some() {
            fields.push("created_at");
        }
        if self.updated_at.is_some() {
            fields.push("updated_at");
        }
        if self.deleted_at.is_some() {
            fields.push("deleted_at");
        }
        if self.uploaded_at.is_some() {
            fields.push("uploaded_at");
        }
        return fields;
    }
    fn apply<O>(
        self,
        mut query: sqlx::query::QueryAs<'_, sqlx::Postgres, O, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::QueryAs<'_, sqlx::Postgres, O, sqlx::postgres::PgArguments> {
        if self.id.is_some() {
            query = query.bind(self.id.unwrap());
        }
        if self.title.is_some() {
            query = query.bind(self.title.unwrap());
        }
        if self.description.is_some() {
            query = query.bind(self.description.unwrap());
        }
        if self.user_id.is_some() {
            query = query.bind(self.user_id.unwrap());
        }
        if self.channel_id.is_some() {
            query = query.bind(self.channel_id.unwrap());
        }
        if self.url.is_some() {
            let value = self.url.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.language.is_some() {
            query = query.bind(self.language.unwrap());
        }
        if self.stage.is_some() {
            query = query.bind(self.stage.unwrap());
        }
        if self.error.is_some() {
            query = query.bind(self.error.unwrap());
        }
        if self.original_url.is_some() {
            query = query.bind(self.original_url.unwrap());
        }
        if self.original_duration.is_some() {
            let value = self.original_duration.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.start_time.is_some() {
            query = query.bind(self.start_time.unwrap());
        }
        if self.end_time.is_some() {
            let value = self.end_time.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.tags.is_some() {
            let value = self.tags.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.created_at.is_some() {
            let value = self.created_at.unwrap();
            query = query.bind(value);
        }
        if self.created_at_start.is_some() && self.created_at_end.is_some() {
            let start = self.created_at_start.unwrap();
            let end = self.created_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.created_at_start.is_some() {
            query = query.bind(self.created_at_start.unwrap());
        } else if self.created_at_end.is_some() {
            query = query.bind(self.created_at_end.unwrap());
        }
        if self.updated_at.is_some() {
            let value = self.updated_at.unwrap();
            query = query.bind(value);
        }
        if self.updated_at_start.is_some() && self.updated_at_end.is_some() {
            let start = self.updated_at_start.unwrap();
            let end = self.updated_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.updated_at_start.is_some() {
            query = query.bind(self.updated_at_start.unwrap());
        } else if self.updated_at_end.is_some() {
            query = query.bind(self.updated_at_end.unwrap());
        }
        if self.deleted_at.is_some() {
            let value = self.deleted_at.unwrap();
            query = query.bind(value);
        }
        if self.deleted_at_start.is_some() && self.deleted_at_end.is_some() {
            let start = self.deleted_at_start.unwrap();
            let end = self.deleted_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.deleted_at_start.is_some() {
            query = query.bind(self.deleted_at_start.unwrap());
        } else if self.deleted_at_end.is_some() {
            query = query.bind(self.deleted_at_end.unwrap());
        }
        if self.uploaded_at.is_some() {
            let value = self.uploaded_at.unwrap();
            query = query.bind(value);
        }
        if self.uploaded_at_start.is_some() && self.uploaded_at_end.is_some() {
            let start = self.uploaded_at_start.unwrap();
            let end = self.uploaded_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.uploaded_at_start.is_some() {
            query = query.bind(self.uploaded_at_start.unwrap());
        } else if self.uploaded_at_end.is_some() {
            query = query.bind(self.uploaded_at_end.unwrap());
        }
        return query;
    }
    fn apply_raw(
        self,
        mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        if self.id.is_some() {
            query = query.bind(self.id.unwrap());
        }
        if self.title.is_some() {
            query = query.bind(self.title.unwrap());
        }
        if self.description.is_some() {
            query = query.bind(self.description.unwrap());
        }
        if self.user_id.is_some() {
            query = query.bind(self.user_id.unwrap());
        }
        if self.channel_id.is_some() {
            query = query.bind(self.channel_id.unwrap());
        }
        if self.url.is_some() {
            let value = self.url.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.language.is_some() {
            query = query.bind(self.language.unwrap());
        }
        if self.stage.is_some() {
            query = query.bind(self.stage.unwrap());
        }
        if self.error.is_some() {
            query = query.bind(self.error.unwrap());
        }
        if self.original_url.is_some() {
            query = query.bind(self.original_url.unwrap());
        }
        if self.original_duration.is_some() {
            let value = self.original_duration.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.start_time.is_some() {
            query = query.bind(self.start_time.unwrap());
        }
        if self.end_time.is_some() {
            let value = self.end_time.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.tags.is_some() {
            let value = self.tags.unwrap();
            if value.is_some() {
                query = query.bind(value.unwrap());
            }
        }
        if self.created_at.is_some() {
            let value = self.created_at.unwrap();
            query = query.bind(value);
        }
        if self.created_at_start.is_some() && self.created_at_end.is_some() {
            let start = self.created_at_start.unwrap();
            let end = self.created_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.created_at_start.is_some() {
            query = query.bind(self.created_at_start.unwrap());
        } else if self.created_at_end.is_some() {
            query = query.bind(self.created_at_end.unwrap());
        }
        if self.updated_at.is_some() {
            let value = self.updated_at.unwrap();
            query = query.bind(value);
        }
        if self.updated_at_start.is_some() && self.updated_at_end.is_some() {
            let start = self.updated_at_start.unwrap();
            let end = self.updated_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.updated_at_start.is_some() {
            query = query.bind(self.updated_at_start.unwrap());
        } else if self.updated_at_end.is_some() {
            query = query.bind(self.updated_at_end.unwrap());
        }
        if self.deleted_at.is_some() {
            let value = self.deleted_at.unwrap();
            query = query.bind(value);
        }
        if self.deleted_at_start.is_some() && self.deleted_at_end.is_some() {
            let start = self.deleted_at_start.unwrap();
            let end = self.deleted_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.deleted_at_start.is_some() {
            query = query.bind(self.deleted_at_start.unwrap());
        } else if self.deleted_at_end.is_some() {
            query = query.bind(self.deleted_at_end.unwrap());
        }
        if self.uploaded_at.is_some() {
            let value = self.uploaded_at.unwrap();
            query = query.bind(value);
        }
        if self.uploaded_at_start.is_some() && self.uploaded_at_end.is_some() {
            let start = self.uploaded_at_start.unwrap();
            let end = self.uploaded_at_end.unwrap();
            query = query.bind(start);
            query = query.bind(end);
        } else if self.uploaded_at_start.is_some() {
            query = query.bind(self.uploaded_at_start.unwrap());
        } else if self.uploaded_at_end.is_some() {
            query = query.bind(self.uploaded_at_end.unwrap());
        }
        return query;
    }
    fn gen_where_statements(&self, param_count: Option<usize>) -> (String, usize) {
        let mut sql = String::new();
        let mut param_count: usize = match param_count {
            Some(param_count) => param_count,
            None => 0,
        };
        let mut and = false;
        if self.id.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} = ${}", "id", param_count));
            } else {
                sql.push_str(&format!(" AND {} = ${}", "id", param_count));
            }
            and = true;
        }
        if self.title.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} LIKE ${}", "title", param_count));
            } else {
                sql.push_str(&format!(" AND {} LIKE ${}", "title", param_count));
            }
            and = true;
        }
        if self.description.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} LIKE ${}", "description", param_count));
            } else {
                sql.push_str(&format!(" AND {} LIKE ${}", "description", param_count));
            }
            and = true;
        }
        if self.user_id.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} = ${}", "user_id", param_count));
            } else {
                sql.push_str(&format!(" AND {} = ${}", "user_id", param_count));
            }
            and = true;
        }
        if self.channel_id.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} = ${}", "channel_id", param_count));
            } else {
                sql.push_str(&format!(" AND {} = ${}", "channel_id", param_count));
            }
            and = true;
        }
        if self.url.is_some() {
            let value = self.url.as_ref().unwrap();
            match value {
                Some(_) => {
                    param_count = param_count + 1;
                    if !and {
                        sql.push_str(&format!("{} LIKE ${}", "url", param_count));
                    } else {
                        sql.push_str(&format!(" AND {} LIKE ${}", "url", param_count));
                    }
                    and = true;
                }
                None => {
                    if !and {
                        sql.push_str(&format!("{} IS NULL", "url"));
                    } else {
                        sql.push_str(&format!(" AND {} IS NULL", "url"));
                    }
                    and = true;
                }
            }
        }
        if self.language.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} LIKE ${}", "language", param_count));
            } else {
                sql.push_str(&format!(" AND {} LIKE ${}", "language", param_count));
            }
            and = true;
        }
        if self.stage.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} = ${}", "stage", param_count));
            } else {
                sql.push_str(&format!(" AND {} = ${}", "stage", param_count));
            }
            and = true;
        }
        if self.error.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} = ${}", "error", param_count));
            } else {
                sql.push_str(&format!(" AND {} = ${}", "error", param_count));
            }
            and = true;
        }
        if self.original_url.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} LIKE ${}", "original_url", param_count));
            } else {
                sql.push_str(&format!(" AND {} LIKE ${}", "original_url", param_count));
            }
            and = true;
        }
        if self.original_duration.is_some() {
            let value = self.original_duration.as_ref().unwrap();
            match value {
                Some(_) => {
                    param_count = param_count + 1;
                    if !and {
                        sql.push_str(&format!("{} LIKE ${}", "original_duration", param_count));
                    } else {
                        sql.push_str(&format!(
                            " AND {} LIKE ${}",
                            "original_duration", param_count
                        ));
                    }
                    and = true;
                }
                None => {
                    if !and {
                        sql.push_str(&format!("{} IS NULL", "original_duration"));
                    } else {
                        sql.push_str(&format!(" AND {} IS NULL", "original_duration"));
                    }
                    and = true;
                }
            }
        }
        if self.start_time.is_some() {
            param_count = param_count + 1;
            if !and {
                sql.push_str(&format!("{} LIKE ${}", "start_time", param_count));
            } else {
                sql.push_str(&format!(" AND {} LIKE ${}", "start_time", param_count));
            }
            and = true;
        }
        if self.end_time.is_some() {
            let value = self.end_time.as_ref().unwrap();
            match value {
                Some(_) => {
                    param_count = param_count + 1;
                    if !and {
                        sql.push_str(&format!("{} LIKE ${}", "end_time", param_count));
                    } else {
                        sql.push_str(&format!(" AND {} LIKE ${}", "end_time", param_count));
                    }
                    and = true;
                }
                None => {
                    if !and {
                        sql.push_str(&format!("{} IS NULL", "end_time"));
                    } else {
                        sql.push_str(&format!(" AND {} IS NULL", "end_time"));
                    }
                    and = true;
                }
            }
        }
        if self.tags.is_some() {
            let value = self.tags.as_ref().unwrap();
            match value {
                Some(_) => {
                    param_count = param_count + 1;
                    if !and {
                        sql.push_str(&format!("{} LIKE ${}", "tags", param_count));
                    } else {
                        sql.push_str(&format!(" AND {} LIKE ${}", "tags", param_count));
                    }
                    and = true;
                }
                None => {
                    if !and {
                        sql.push_str(&format!("{} IS NULL", "tags"));
                    } else {
                        sql.push_str(&format!(" AND {} IS NULL", "tags"));
                    }
                    and = true;
                }
            }
        }
        if self.created_at.is_some() {
            param_count = param_count + 1;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!("{} = ${}", "created_at", param_count));
        }
        if self.created_at_start.is_some() && self.created_at_end.is_some() {
            param_count = param_count + 2;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!(
                "{} BETWEEN ${} AND ${}",
                "created_at",
                param_count - 1,
                param_count
            ));
        } else if self.created_at_start.is_some() {
            param_count = param_count + 1;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!("{} >= ${}", "created_at", param_count));
        } else if self.created_at_end.is_some() {
            param_count = param_count + 1;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!("{} <= ${}", "created_at", param_count));
        }
        if self.updated_at.is_some() {
            param_count = param_count + 1;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!("{} = ${}", "updated_at", param_count));
        }
        if self.updated_at_start.is_some() && self.updated_at_end.is_some() {
            param_count = param_count + 2;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!(
                "{} BETWEEN ${} AND ${}",
                "updated_at",
                param_count - 1,
                param_count
            ));
        } else if self.updated_at_start.is_some() {
            param_count = param_count + 1;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!("{} >= ${}", "updated_at", param_count));
        } else if self.updated_at_end.is_some() {
            param_count = param_count + 1;
            if and {
                sql.push_str(" AND ");
            }
            and = true;
            sql.push_str(&format!("{} <= ${}", "updated_at", param_count));
        }
        if self.deleted_at.is_some() {
            let value = self.deleted_at.as_ref().unwrap();
            if value.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} = ${}", "deleted_at", param_count));
            }
        } else {
            if !and {
                sql.push_str(&format!("{} IS NULL", "deleted_at"));
            } else {
                sql.push_str(&format!(" AND {} IS NULL", "deleted_at"));
            }
            and = true;
        }
        if self.deleted_at_start.is_some() && self.deleted_at_end.is_some() {
            let start = self.deleted_at_start.as_ref().unwrap();
            let end = self.deleted_at_end.as_ref().unwrap();
            if start.is_some() && end.is_some() {
                param_count = param_count + 2;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!(
                    "{} BETWEEN ${} AND ${}",
                    "deleted_at",
                    param_count - 1,
                    param_count
                ));
            } else if start.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} >= ${}", "deleted_at", param_count));
            } else if end.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} <= ${}", "deleted_at", param_count));
            }
        } else if self.deleted_at_start.is_some() {
            let start = self.deleted_at_start.as_ref().unwrap();
            if start.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} >= ${}", "deleted_at", param_count));
            }
        } else if self.deleted_at_end.is_some() {
            let end = self.deleted_at_end.as_ref().unwrap();
            if end.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} <= ${}", "deleted_at", param_count));
            }
        }
        if self.uploaded_at.is_some() {
            let value = self.uploaded_at.as_ref().unwrap();
            if value.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} = ${}", "uploaded_at", param_count));
            }
        } else {
            if !and {
                sql.push_str(&format!("{} IS NULL", "uploaded_at"));
            } else {
                sql.push_str(&format!(" AND {} IS NULL", "uploaded_at"));
            }
            and = true;
        }
        if self.uploaded_at_start.is_some() && self.uploaded_at_end.is_some() {
            let start = self.uploaded_at_start.as_ref().unwrap();
            let end = self.uploaded_at_end.as_ref().unwrap();
            if start.is_some() && end.is_some() {
                param_count = param_count + 2;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!(
                    "{} BETWEEN ${} AND ${}",
                    "uploaded_at",
                    param_count - 1,
                    param_count
                ));
            } else if start.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} >= ${}", "uploaded_at", param_count));
            } else if end.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} <= ${}", "uploaded_at", param_count));
            }
        } else if self.uploaded_at_start.is_some() {
            let start = self.uploaded_at_start.as_ref().unwrap();
            if start.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} >= ${}", "uploaded_at", param_count));
            }
        } else if self.uploaded_at_end.is_some() {
            let end = self.uploaded_at_end.as_ref().unwrap();
            if end.is_some() {
                param_count = param_count + 1;
                if and {
                    sql.push_str(" AND ");
                }
                and = true;
                sql.push_str(&format!("{} <= ${}", "uploaded_at", param_count));
            }
        }
        return (sql, param_count);
    }
}
impl crate::database::queries::filter::Filterable for Video {
    type F = InternalFiltrationVideoFilters;
}
