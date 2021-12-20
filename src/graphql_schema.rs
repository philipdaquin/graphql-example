extern crate dotenv;
use std::env;
use dotenv::dotenv;



use diesel::{query_builder::Query, Queryable, PgConnection, Connection, RunQueryDsl, Insertable};
use juniper::{EmptyMutation, RootNode};


#[derive(Queryable)]
pub struct Member { 
    pub id: i32,
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

#[juniper::object(description = "Team Member")]
impl Member { 
    pub fn get_id(&self) -> i32 { 
        self.id
    }
    pub fn get_name(&self) -> &str { 
        self.name.as_str()
    }
    pub fn get_knockouts(&self) -> i32 { 
        self.knockouts
    }
    pub fn get_teamid(&self) -> i32 { 
        self.team_id
    }
}

pub struct QueryRoot;
fn establish_connection() -> PgConnection { 
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Database_URL could not be found");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


//  graphq_object allows us to define the fields on the different nodes throughout our schema
#[juniper::object()]
impl QueryRoot { 
   fn members() -> Vec<Member> { 
       use crate::schema::members::dsl::*;
       let connection = establish_connection();
       members 
        .limit(100)
        .load::<Member>(&connection)
        .expect("Error Loading Members")
   }
   fn teams() -> Vec<Team> { 
       use crate::schema::teams::dsl::*;
       let connection = establish_connection();
       teams
        .limit(10)
        .load<Team>(&connection)
        .expect("Error Loading Teams")    

   }
}
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
pub fn create_schema() -> Schema { 
    Schema::new(QueryRoot {}, MutationRoot {},)
}

#[derive(Queryable)]
pub struct Team { 
    pub id: i32,
    pub name: String,
}
#[juniper::object(description = "A team of members")]
impl Team { 
    pub fn get_id(&self) -> i32 { 
        self.id
    }
    pub fn get_name(&self) -> &str { 
        self.name.as_str()
    }
    pub fn get_members(&self) -> Vec<Member> { 
        vec![];
        use crate::schema::members::dsl::*;
        let connection = establish_connection();
        members
            .filter(team_id.eq(self.id))
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error loading Members")
    }
}
pub struct MutationRoot;

#[juniper::object]
impl MutationRoot { 

    fn create_member(data: NewMember) -> Member { 
        let connection = establish_connection();
        diesel::insert_into(members::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error Saving new Post")

    }
}

//  GraphQLinputObject --> to create an input object for our GraphQl schema
//  Insertable --> let diesel know that this struct is vlaid input for an insertion SQL statement
#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "members"]
pub struct NewMember { 
    pub name: String, 
    pub knockouts: i32, 
    pub team_id: i32
}

