use diesel::prelude::*;
use diesel::result::Error;
use rocket_contrib::{Json, Value, Template};
use rocket::http::{Header, Status, RawStr};
use rocket::request::{FromFormValue};
use diesel::helper_types::Filter;

use models::*;
use guards::*;
use db::DbConn;

#[derive(Debug)]
pub enum ParentFilter {
    Id(i32),
    Null,
}

impl<'v> FromFormValue<'v> for ParentFilter {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<ParentFilter, &'v RawStr> {
        
        match value.as_str() {
            "null" => return Ok(ParentFilter::Null),
            _ => {
                let result = value.parse::<i32>();
                if result.is_ok() {
                    return Ok(ParentFilter::Id(result.unwrap()));
                }
                return Err(value);
            }
        }
    
    }
}

#[derive(FromForm, Debug)]
pub struct TaskFilters { 
    pub completed: Option<bool>,
    pub parent: Option<ParentFilter>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct PagedTasks {
    pub page: i64,
    pub more: bool,
    pub results: Vec<Task>
}

pub enum TaskQueryResult {
    Count(i64),
    Rows(i64, i64, Result<Vec<Task>, Error>)
}

fn task_query(conn: &DbConn, user: &User, filters: &Option<TaskFilters>, count: bool) -> TaskQueryResult {
     use ::schema::tasks::dsl::*;

    let mut page = 1;
    let mut limit = 20;
   

    let mut query = Task::belonging_to(user).into_boxed();
    if let Some(filters) = filters {
        if let Some(f_completed) = filters.completed {
            query = query.filter(completed.eq(f_completed))
        }

        if let Some(ref f_parent) = filters.parent {
            query = match f_parent {
                ParentFilter::Id(i) => query.filter(parent_id.nullable().eq(i)),
                ParentFilter::Null => query.filter(parent_id.nullable().is_null()),
            }
        }

        if let Some(p) = filters.page {
            page = p
        }

        if let Some(l) = filters.limit {
            limit = l
        }
    }
    if count {
        TaskQueryResult::Count(query.count().get_result(&conn.pool).unwrap_or(0))
    } else {
        let rows = query
            .limit(limit)
            .offset((page - 1) * limit)
            .load::<Task>(&conn.pool);
        TaskQueryResult::Rows(page, limit, rows)
    }
}

fn run_task_query(conn: DbConn, user: User, filters: Option<TaskFilters>) -> Result<Json<PagedTasks>, Error> {
    
    let mut total = 0;
    
    println!("{:?}", filters);

    let query_results = task_query(&conn, &user, &filters, false);
    if let TaskQueryResult::Count(count) = task_query(&conn, &user, &filters, true){
        total = count;
    }

    match query_results {
        TaskQueryResult::Rows(page, limit, result) => {
            match result {
                Ok(t) => {
                    let response = PagedTasks {
                        page: page,
                        more: page * limit < total,
                        results: t 
                    };
                    Ok(Json(response))
                },
                Err(e) => Err(e)
            }
        }
        _ => Err(Error::NotFound)
    }    
}

#[get("/tasks?<filters>")]
pub fn get_tasks(conn: DbConn, auth: AccessTokenAuth, filters: Option<TaskFilters>) -> Result<Json<PagedTasks>, Error> {
    return run_task_query(conn, auth.user, filters)
}

#[get("/tasks/top?<filters>")]
pub fn get_tasks_toplevel(conn: DbConn, auth: AccessTokenAuth, filters: Option<TaskFilters>) -> Result<Json<PagedTasks>, Error> {
    if let Some(mut filters) = filters {
        filters.parent = Option::Some(ParentFilter::Null);
        return run_task_query(conn, auth.user, Option::Some(filters))
    }
    run_task_query(conn, auth.user, Option::Some(TaskFilters {
        parent: Option::Some(ParentFilter::Null),
        completed: Option::None,
        page: Option::None,
        limit: Option::None
    }))
}

#[get("/tasks/in/<parent>?<filters>")]
pub fn get_tasks_belongingto(parent: i32, conn: DbConn, auth: AccessTokenAuth, filters: Option<TaskFilters>) -> Result<Json<PagedTasks>, Error> {
    if let Some(mut filters) = filters {
        filters.parent = Option::Some(ParentFilter::Id(parent));
        return run_task_query(conn, auth.user, Option::Some(filters))
    }
    run_task_query(conn, auth.user, Option::Some(TaskFilters {
        parent: Option::Some(ParentFilter::Id(parent)),
        completed: Option::None,
        page: Option::None,
        limit: Option::None
    }))
}

#[get("/tasks/active?<filters>")]
pub fn get_tasks_active(conn: DbConn, auth: AccessTokenAuth, filters: Option<TaskFilters>) -> Result<Json<PagedTasks>, Error> {
    if let Some(mut filters) = filters {
        filters.completed = Option::Some(false);
        return run_task_query(conn, auth.user, Option::Some(filters))
    }
    run_task_query(conn, auth.user, Option::Some(TaskFilters {
        parent: Option::None,
        completed: Option::Some(false),
        page: Option::None,
        limit: Option::None
    }))
}


#[get("/tasks/completed?<filters>")]
pub fn get_tasks_completed(conn: DbConn, auth: AccessTokenAuth, filters: Option<TaskFilters>) -> Result<Json<PagedTasks>, Error> {
    if let Some(mut filters) = filters {
        filters.completed = Option::Some(true);
        return run_task_query(conn, auth.user, Option::Some(filters))
    }
    run_task_query(conn, auth.user, Option::Some(TaskFilters {
        parent: Option::None,
        completed: Option::Some(true),
        page: Option::None,
        limit: Option::None
    }))
}
