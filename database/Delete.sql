DEFINE TABLE udelete SCHEMAFULL;
DEFINE FIELD identifier ON TABLE udelete TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON TABLE udelete TYPE string ASSERT $value != NONE;
DEFINE FIELD generation_time ON TABLE udelete TYPE string ASSERT $value != NONE;
