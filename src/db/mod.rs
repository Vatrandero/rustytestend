pub mod error;
pub(super) mod commons{
pub use crate::cfg::DBPGCfg;
pub use crate::models;
pub use crate::models::dtos;
pub use crate::models::dtos::UserRegisterReq;
pub use crate::models::users::*;
pub use async_trait::async_trait; // make traits with async dyn-compatable.
pub use passhash::hash_password;
pub use std::{boxed::Box, fmt::format};
pub use std::error::Error;
pub use tokio::stream::*;
pub use uuid::Uuid;
}
use commons::*;
use error::DBError;
pub mod pgsql;

/*
    Q: Why there are so many traits?
    A: Considered that for different data - will be used different DMBS.
    But it will b enden by using Postgressql for moost of data -
    and using redis for sessions.

*/

#[async_trait]
pub trait UsersManager {
    async fn register_new_user(&self, u: &UserRegisterReq) -> Result<(), Box<dyn Error>>;
    async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, Box<dyn Error>>;

}
#[async_trait]
pub trait UsersSessionManager {
    async fn register_new_session(
        &self,
        u: &User,
    ) -> Result<Uuid, Box<dyn Error>>;
    /// Resolves users session cookie Uuid into user id.
    async fn resolve_user_session_to_id(&self, uuid: Uuid) -> Result<Option<i32>, Box<dyn Error>>; // i64?

    /// Uses if user unlogined.
    async fn end_session(&self, id: i32) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait KTestManager {
    /// creates new tet on database side.
    async fn create_new(&self) -> Result<(), DBError>;

    // List and search


    async fn list_tests_meta_last_n(&self, n: i32)
    -> Result<Vec<models::knowledge_test::KnowledgeTestMeta>, DBError>  ;

    async fn list_simple_by_search_text(&self, text: &str)
    -> Result<Vec<models::knowledge_test::KnowledgeTestMeta>, DBError>;
    
    async fn select_test_by_id(&self, id: i32) 
    -> Result<models::knowledge_test::KnolewdgeTestPriv, DBError>; 

      async fn select_test_priv_by_id(&self, id: i32) 
    -> Result<models::knowledge_test::KnolewdgeTestPriv, DBError>; 
    
      async fn select_test_meta_by_id(&self, id: i32) 
    -> Result<models::knowledge_test::KnowledgeTestMeta, DBError>; 


    async fn delete(&self, test_id: i64) -> Result<(), DBError >;

    async fn asign(&self, asign: models::dtos::UnAsignReq);
    async fn get_asign_by_id(
        &self,
        user_id: i64,
        test_id: i64,
    ) -> Result<models::knowledge_test::KtAsigment, Box<dyn Error>>;

    /// this method needs to be called when starting new KtESTsESSION.
    /// decrease tries for given asigment in DB
    async fn decrease_asignment(&self, asign: models::knowledge_test::KtAsigment);

    async fn unasign(&self, unasign: models::dtos::UnAsignReq)
    -> Result<(), DBError>;
}
#[async_trait]
pub trait KTestSessionManager {
    /// Tries for given asigment run the new test session.
    async fn new(
        &self,
        asign: models::knowledge_test::KtAsigment,
    ) -> Result<models::knowledge_test::KTestOngoing, Box<dyn Error>>;

    #[deprecated = "This method exists because of problem 
    occured on traits design.
    See doc comment for explanation."]
    /// Cancels given test session, deletes
    /// session without traces.
    /// Use this method to rollback bad session.
    /// This is not same as `end_session`, wich just marks
    /// session ended, completed.
    ///
    /// # DEPRECATED Reason:
    /// When designed this KTEstSessionManager, in considered
    /// that this may different DBMS (Redis for caching, as example)
    /// There are some solutions usable.
    ///
    /// One solutionn is to make this method accept `&dyn KTestManager`
    /// and call it's methods from here.
    /// Other solution is to merge traits.
    /// And  another curretnly used - to fully seperate
    /// traits and make caller be responsible for tracking
    /// both managers acted succusuflly.
    ///
    /// Tha last solution: Create one more entity, that will be 
    /// controlling transaction of session starting.
    /// If it nas started - commit, if not - rollback.
    /// 
    /// In all ways, seems like we also should use UUID instead of INTEGER.
    ///    
    async fn cancel(&self);

    /// Takes updated `KTestOngoing`, considered
    /// it has modifications by user.
    /// Yhis method valodates original test data and tries
    /// to commit changes.
    async fn update(&self, ko: models::knowledge_test::KTestOngoing) -> Result<(), Box<dyn Error>>;
    /// Succusuflly end sesion.
    async fn end_session(&self, id: i64) -> Result<(), Box<dyn Error>>;
}
