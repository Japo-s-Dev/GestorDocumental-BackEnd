use sea_query::Iden;

#[derive(Iden)]
pub enum EventIden {
	#[iden = "event"]
	Table,
	Id,
	UserId,
	Action,
	Object,
	ObjectId,
	Timestamp,
	OldData,
	NewData,
	AdditionalInfo,
}

#[derive(Iden)]
pub enum UserIden {
	#[iden = "user"]
	Table,
	Id,
	Username,
}

#[derive(Iden)]
pub enum IndexIden {
	#[iden = "index"]
	Table,
	Id,
	ProjectId,
	Required,
	IndexName,
	DatatypeId,
}

#[derive(Iden)]
pub enum DatatypeIden {
	#[iden = "datatype"]
	Table,
	Id,
	DatatypeName,
}
