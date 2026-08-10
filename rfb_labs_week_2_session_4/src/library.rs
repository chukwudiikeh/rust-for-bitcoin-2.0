use crate::catalogue::Item;
use crate::error::LibraryError;
use crate::member::Member;
use crate::catalogue::LoanStatus;

pub const MAX_ITEMS_PER_MEMBER: usize = 3;

/// Owns every item and every member.
///
/// The fields are private because the library is responsible for keeping an
/// item's `LoanStatus` and a member's borrowed-id list in agreement. Callers
/// reach the data through the borrowing lookups below.
// TODO(Part 3): delete this attribute once your lookups actually read the
// fields. It is here only so the untouched starter crate compiles clean.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Library {
    items: Vec<Item>,
    members: Vec<Member>,
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), LibraryError> {
      if item.title.is_empty(){
        return( Err(LibraryError::EmptyTitle));
      }

      if self.items.iter().any(|existing| existing.id == item.id){
        return(Err(LibraryError::DuplicateItemId{id:item.id}));
      }
      
      self.items.push(item);
      Ok(())
    }

    pub fn register_member(&mut self, member: Member) -> Result<(), LibraryError> {
        // TODO(Part 3): move `member` in. Reject an id already registered.
       if self.members.iter().any(|existing| existing.id == member.id){
        return(Err(LibraryError::DuplicateMemberId{id:member.id}));
       }

       self.members.push(member);
       Ok(())
    }

    pub fn find_item(&self, id: u32) -> Option<&Item> {
        // TODO(Part 3): borrow from `self`; do not clone.
        self.items.iter().find(|existing| existing.id == id)
    }

    pub fn find_member(&self, id: u32) -> Option<&Member> {
        // TODO(Part 3)
        self.members.iter().find(|existing| existing.id == id)
    }

    pub fn items_by_author<'a>(&'a self, author: &str) -> Vec<&'a Item> {
        // TODO(Part 3): return references to all matching items.
       self.items.iter().filter(|item| item.author == author).collect()
    }

    pub fn available_items(&self) -> Vec<&Item> {
        // TODO(Part 3)
       self.items.iter().filter(|item| item.status == LoanStatus::Available).collect()
    }

    pub fn longest_loan_item(&self) -> Option<&Item> {
        self.items.iter().max_by_key(|item| item.loan_days())
    }

    pub fn checkout(&mut self, item_id: u32, member_id: u32, day: u32) -> Result<(), LibraryError> {
        // TODO(Part 5): validate in the order given in ASSIGNMENT.md, then
        // update the item's status and the member's list together.
      let item = self.find_item(item_id).ok_or(LibraryError::ItemNotFound{id:item_id})?;
      let member = self.find_member(member_id).ok_or(LibraryError::MemberNotFound{id:member_id})?;

      if let LoanStatus::Lost = item.status{
        return Err (LibraryError::ItemIsLost{id:item_id});
      }

      if let LoanStatus::OnLoan{member_id: current_borrower,..} = item.status{
        return Err (LibraryError::ItemAlreadyOnLoan{id:item_id,member_id:current_borrower});
      }
      if member.borrowed_item_ids.len() >= MAX_ITEMS_PER_MEMBER{
        return Err(LibraryError::BorrowLimitReached{member_id,limit:MAX_ITEMS_PER_MEMBER});
      }

      let item =self.items.iter_mut().find(|i| i.id == item_id).unwrap();
      let member =self.members.iter_mut().find(|i| i.id == member_id).unwrap();

      item.status = LoanStatus::OnLoan{member_id,day_borrowed:day};
      member.borrowed_item_ids.push(item_id);

      Ok(())
    }

    /// Returns the late fee owed, in cents.
    pub fn return_item(&mut self, item_id: u32, day: u32) -> Result<u32, LibraryError> {
        // TODO(Part 6): checked subtraction must return InvalidReturnDay.
        let _ = (item_id, day);
        todo!("return an item")
    }
}
