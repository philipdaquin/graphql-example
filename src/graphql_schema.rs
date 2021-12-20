use diesel::query_builder::Query;
use juniper::{EmptyMutation, RootNode};


#[derive(Clone)]
struct Member { 
    id: i32,
    name: String
}

#[juniper::object(description = "Team Member")]
impl Member { 
    pub fn get_id(&self) -> i32 { 
        self.id
    }
    pub fn get_name(&self) -> &str { 
        self.name.as_str()
    }
}

pub struct QueryRoot;
//  graphq_object allows us to define the fields on the different nodes throughout our schema
#[juniper::object()]
impl QueryRoot { 
   fn members() -> Vec<Member> { 
       vec![
           Member { 
               id: 1,
               name: "lol!".to_owned()
           },
           Member { 
               id: 2,
               name: "String!".to_owned()
           }
       ]
   }
}
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;
pub fn create_schema() -> Schema { 
    Schema::new(QueryRoot {}, EmptyMutation::new(),)
}

