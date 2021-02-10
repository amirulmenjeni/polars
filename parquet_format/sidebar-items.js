initSidebarItems({"enum":[["BoundaryOrder","Enum to annotate whether lists of min/max elements inside ColumnIndex are ordered and if so, in which direction."],["ColumnOrder",""],["CompressionCodec","Supported compression algorithms."],["ConvertedType","Common types used by frameworks(e.g. hive, pig) using parquet.  This helps map between types in those frameworks to the base types in parquet.  This is only metadata and not needed to read or write the data."],["Encoding","Encodings supported by Parquet.  Not all encodings are valid for all types.  These enums are also used to specify the encoding of definition and repetition levels. See the accompanying doc for the details of the more complicated encodings."],["FieldRepetitionType","Representation of Schemas"],["LogicalType",""],["PageType",""],["TimeUnit",""],["Type","Types supported by Parquet.  These types are intended to be used in combination with the encodings to control the on disk storage format. For example INT16 is not included as a type since a good encoding of INT32 would handle this."]],"struct":[["BsonType","Embedded BSON logical type annotation"],["ColumnChunk",""],["ColumnIndex","Description for ColumnIndex. Each [i] refers to the page at OffsetIndex.page_locations[i]"],["ColumnMetaData","Description for column metadata"],["DataPageHeader","Data page header"],["DataPageHeaderV2","New page format allowing reading levels without decompressing the data Repetition and definition levels are uncompressed The remaining section containing the data is compressed if is_compressed is true"],["DateType",""],["DecimalType","Decimal logical type annotation"],["DictionaryPageHeader","TODO: *"],["EnumType",""],["FileMetaData","Description for file metadata"],["IndexPageHeader",""],["IntType","Integer logical type annotation"],["JsonType","Embedded JSON logical type annotation"],["KeyValue","Wrapper struct to store key values"],["ListType",""],["MapType",""],["MicroSeconds",""],["MilliSeconds","Time units for logical types"],["NanoSeconds",""],["NullType","Logical type to annotate a column that is always null."],["OffsetIndex",""],["PageEncodingStats","statistics of a given page type and encoding"],["PageHeader",""],["PageLocation",""],["RowGroup",""],["SchemaElement","Represents a element inside a schema definition."],["SortingColumn","Wrapper struct to specify sort order"],["Statistics","Statistics per row group and per page All fields are optional."],["StringType","Empty structs to use as logical type annotations"],["TimeType","Time logical type annotation"],["TimestampType","Timestamp logical type annotation"],["TypeDefinedOrder","Empty struct to signal the order defined by the physical or logical type"],["UUIDType",""]]});