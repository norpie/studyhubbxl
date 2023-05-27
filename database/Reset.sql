DEFINE TABLE reset SCHEMAFULL;
DEFINE FIELD identifier ON TABLE reset TYPE string ASSERT $value != NONE;
DEFINE FIELD slug ON TABLE reset TYPE string ASSERT $value != NONE;
DEFINE FIELD generation_time ON TABLE reset TYPE string ASSERT $value != NONE;
