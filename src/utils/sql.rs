pub fn query_tables<'a>() -> &'a str {
    r#"
            SELECT tbl.name AS table_name
            FROM sys.tables tbl
            WHERE tbl.is_ms_shipped = 0
                AND tbl.type = 'U'
            ORDER BY tbl.name;
	"#
}

pub fn query_colums<'a>() -> &'a str {
    r#"
		SELECT c.name                                                   AS column_name,
		CASE typ.is_assembly_type
			WHEN 1 THEN TYPE_NAME(c.user_type_id)
			ELSE TYPE_NAME(c.system_type_id)
		END                                                             AS data_type,
		COLUMNPROPERTY(c.object_id, c.name, 'charmaxlen')               AS character_maximum_length,
		ISNULL(OBJECT_DEFINITION(c.default_object_id), '')              AS column_default,
		c.is_nullable                                                   AS is_nullable,
		COLUMNPROPERTY(c.object_id, c.name, 'IsIdentity')               AS is_identity,
		OBJECT_NAME(c.object_id)                                        AS table_name,
		OBJECT_NAME(c.default_object_id)                                AS constraint_name,
		convert(tinyint, CASE
		WHEN c.system_type_id IN (48, 52, 56, 59, 60, 62, 106, 108, 122, 127) THEN c.precision
		END) AS numeric_precision,
		convert(int, CASE
		WHEN c.system_type_id IN (40, 41, 42, 43, 58, 61) THEN NULL
		ELSE ODBCSCALE(c.system_type_id, c.scale) END) AS numeric_scale
	FROM sys.columns c
		INNER JOIN sys.tables t ON c.object_id = t.object_id
		INNER JOIN sys.types typ ON c.user_type_id = typ.user_type_id
	WHERE OBJECT_NAME(c.object_id) = @P1 
	AND t.is_ms_shipped = 0
	ORDER BY table_name, COLUMNPROPERTY(c.object_id, c.name, 'ordinal');  
	"#
}
