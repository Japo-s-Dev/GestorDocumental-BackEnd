use sea_query::Iden;

#[derive(Iden)]
pub enum CommonIden {
	Id,
	IsDeleted,
}

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
pub enum DocumentIden {
	#[iden = "document"]
	Table,
	Id,
	Name,
	ArchiveId,
}

#[derive(Iden)]
enum RoleIden {
	Id,
	RoleName,
	Pwd,
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

#[derive(Iden)]
pub enum AssociatedPrivilegeIden {
	#[iden = "associated_privilege"]
	Table,
	Id,
	RoleName,
	#[iden = "privilege_id"]
	PrivilegeId,
	IsEnabled,
}

#[derive(Iden)]
pub enum StructurePrivilegeIden {
	#[iden = "structure_privilege"]
	Table,
	Id,
	UserId,
	ProjectId,
	IsEnabled,
}

#[derive(Iden)]
pub enum StructureIden {
	#[iden = "structure"]
	Table,
	Id,
	ProjectName,
	IsDeleted,
}
