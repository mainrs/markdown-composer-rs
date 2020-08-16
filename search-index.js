var searchIndex = JSON.parse('{\
"markdown_composer":{"doc":"","i":[[0,"builders","markdown_composer","Contains builders for all Markdown content types.",null,null],[0,"image","markdown_composer::builders","",null,null],[3,"ImageBuilder","markdown_composer::builders::image","",null,null],[11,"new","","",0,[[]]],[11,"text","","",0,[[]]],[11,"url","","",0,[[]]],[11,"build","","",0,[[],["image",3]]],[0,"link","markdown_composer::builders","",null,null],[3,"LinkBuilder","markdown_composer::builders::link","",null,null],[11,"new","","",1,[[]]],[11,"text","","",1,[[]]],[11,"url","","",1,[[]]],[11,"inlined","","",1,[[]]],[11,"build","","",1,[[],["link",3]]],[0,"list","markdown_composer::builders","",null,null],[3,"ListBuilder","markdown_composer::builders::list","",null,null],[11,"new","","",2,[[]]],[11,"add","","",2,[[]]],[11,"ordered","","",2,[[],["list",3]]],[11,"unordered","","",2,[[],["list",3]]],[0,"extensions","markdown_composer","",null,null],[0,"github","markdown_composer::extensions","",null,null],[3,"CheckmarkItem","markdown_composer::extensions::github","",null,null],[12,"checked","","",3,null],[12,"text","","",3,null],[8,"Strikethrough","","",null,null],[10,"strikethrough","","",4,[[],["string",3]]],[11,"new","","",3,[[]]],[11,"from","","",3,[[]]],[0,"traits","markdown_composer","",null,null],[8,"MarkdownElement","markdown_composer::traits","",null,null],[10,"render","","",5,[[],["string",3]]],[0,"transforms","markdown_composer","",null,null],[8,"BlockQuote","markdown_composer::transforms","",null,null],[10,"block_quote","","",6,[[],["string",3]]],[10,"block_quote_multi_line","","",6,[[],["string",3]]],[8,"Bold","","",null,null],[10,"bold","","",7,[[],["string",3]]],[8,"CodeBlock","","",null,null],[10,"code_block","","",8,[[],["string",3]]],[10,"code_block_with_language","","",8,[[["asref",8]],["string",3]]],[8,"Inline","","",null,null],[10,"inline","","",9,[[],["string",3]]],[8,"Italic","","",null,null],[10,"italic","","",10,[[],["string",3]]],[0,"types","markdown_composer","",null,null],[0,"header","markdown_composer::types","",null,null],[3,"HeaderLevel","markdown_composer::types::header","The level of a header.",null,null],[3,"Header","","A header.",null,null],[12,"text","","The header text.",11,null],[12,"level","","The header level.",11,null],[11,"new","","Returns a default header level of 1.",12,[[]]],[11,"from","","Creates a new header level.",12,[[]]],[11,"new","","Creates a new empty header with a level of 1.",11,[[]]],[11,"from","","Creates a new header.",11,[[]]],[0,"image","markdown_composer::types","",null,null],[3,"Image","markdown_composer::types::image","",null,null],[12,"text","","",13,null],[12,"url","","",13,null],[11,"new","","",13,[[]]],[11,"from","","",13,[[]]],[0,"link","markdown_composer::types","",null,null],[3,"Link","markdown_composer::types::link","",null,null],[12,"text","","",14,null],[12,"url","","",14,null],[12,"inlined","","",14,null],[11,"new","","",14,[[]]],[11,"from","","",14,[[]]],[0,"list","markdown_composer::types","",null,null],[3,"List","markdown_composer::types::list","",null,null],[12,"items","","",15,null],[12,"ordered","","",15,null],[6,"ListItem","","",null,null],[11,"new","","",15,[[]]],[11,"ordered","","",15,[[]]],[11,"ordered_with","","",15,[[["listitem",6],["vec",3]]]],[11,"unordered","","",15,[[]]],[11,"unordered_with","","",15,[[["listitem",6],["vec",3]]]],[11,"add","","",15,[[["listitem",6]]]],[0,"markdown","markdown_composer::types","",null,null],[3,"Markdown","markdown_composer::types::markdown","",null,null],[12,"elements","","",16,null],[11,"new","","",16,[[]]],[11,"with","","",16,[[["vec",3],["box",3]]]],[11,"with_remark","","Creates a new markdown file, pre-populating it with the…",16,[[]]],[11,"header","","",16,[[]]],[11,"header1","","",16,[[]]],[11,"header2","","",16,[[]]],[11,"header3","","",16,[[]]],[11,"header4","","",16,[[]]],[11,"header5","","",16,[[]]],[11,"header6","","",16,[[]]],[11,"list","","",16,[[["list",3]]]],[11,"link","","",16,[[["link",3]]]],[11,"paragraph","","",16,[[]]],[11,"render","","",16,[[],["string",3]]],[0,"paragraph","markdown_composer::types","",null,null],[3,"Paragraph","markdown_composer::types::paragraph","",null,null],[12,"text","","",17,null],[11,"from","","",17,[[]]],[17,"PRELIMINARY_REMARK","markdown_composer","",null,null],[11,"from","markdown_composer::builders::image","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"clone_box","","",0,[[]]],[11,"from","markdown_composer::builders::link","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"clone_box","","",1,[[]]],[11,"from","markdown_composer::builders::list","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"clone_box","","",2,[[]]],[11,"render","markdown_composer::extensions::github","",3,[[],["string",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"to_string","","",3,[[],["string",3]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"clone_box","","",3,[[]]],[11,"from","markdown_composer::types::header","",12,[[]]],[11,"into","","",12,[[]]],[11,"to_owned","","",12,[[]]],[11,"clone_into","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"clone_box","","",12,[[]]],[11,"render","","",11,[[],["string",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"to_owned","","",11,[[]]],[11,"clone_into","","",11,[[]]],[11,"to_string","","",11,[[],["string",3]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"clone_box","","",11,[[]]],[11,"render","markdown_composer::types::image","",13,[[],["string",3]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"to_owned","","",13,[[]]],[11,"clone_into","","",13,[[]]],[11,"to_string","","",13,[[],["string",3]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"clone_box","","",13,[[]]],[11,"render","markdown_composer::types::link","",14,[[],["string",3]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"to_string","","",14,[[],["string",3]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"clone_box","","",14,[[]]],[11,"render","markdown_composer::types::list","",15,[[],["string",3]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"to_string","","",15,[[],["string",3]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"clone_box","","",15,[[]]],[11,"from","markdown_composer::types::markdown","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_string","","",16,[[],["string",3]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"render","markdown_composer::types::paragraph","",17,[[],["string",3]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"to_owned","","",17,[[]]],[11,"clone_into","","",17,[[]]],[11,"to_string","","",17,[[],["string",3]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"clone_box","","",17,[[]]],[11,"from","markdown_composer::types::header","",12,[[]]],[11,"from","markdown_composer::types::list","",15,[[["list",3]]]],[11,"clone","markdown_composer::builders::image","",0,[[],["imagebuilder",3]]],[11,"clone","markdown_composer::builders::link","",1,[[],["linkbuilder",3]]],[11,"clone","markdown_composer::builders::list","",2,[[],["listbuilder",3]]],[11,"clone","markdown_composer::extensions::github","",3,[[],["checkmarkitem",3]]],[11,"clone","markdown_composer::types::header","",12,[[],["headerlevel",3]]],[11,"clone","","",11,[[],["header",3]]],[11,"clone","markdown_composer::types::image","",13,[[],["image",3]]],[11,"clone","markdown_composer::types::link","",14,[[],["link",3]]],[11,"clone","markdown_composer::types::list","",15,[[],["list",3]]],[11,"clone","markdown_composer::types::paragraph","",17,[[],["paragraph",3]]],[11,"default","markdown_composer::builders::image","",0,[[],["imagebuilder",3]]],[11,"default","markdown_composer::builders::link","",1,[[],["linkbuilder",3]]],[11,"default","markdown_composer::builders::list","",2,[[],["listbuilder",3]]],[11,"default","markdown_composer::extensions::github","",3,[[],["checkmarkitem",3]]],[11,"default","markdown_composer::types::header","Returns the default header level (1).",12,[[]]],[11,"default","","",11,[[],["header",3]]],[11,"default","markdown_composer::types::image","",13,[[],["image",3]]],[11,"default","markdown_composer::types::link","",14,[[],["link",3]]],[11,"default","markdown_composer::types::list","",15,[[],["list",3]]],[11,"default","markdown_composer::types::markdown","",16,[[],["markdown",3]]],[11,"eq","markdown_composer::builders::image","",0,[[["imagebuilder",3]]]],[11,"ne","","",0,[[["imagebuilder",3]]]],[11,"eq","markdown_composer::builders::link","",1,[[["linkbuilder",3]]]],[11,"ne","","",1,[[["linkbuilder",3]]]],[11,"eq","markdown_composer::types::header","",12,[[["headerlevel",3]]]],[11,"ne","","",12,[[["headerlevel",3]]]],[11,"eq","","",11,[[["header",3]]]],[11,"ne","","",11,[[["header",3]]]],[11,"fmt","markdown_composer::builders::image","",0,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::builders::link","",1,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::builders::list","",2,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::extensions::github","",3,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::header","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::image","",13,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::link","",14,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::list","",15,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::paragraph","",17,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::extensions::github","",3,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::header","",11,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::image","",13,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::link","",14,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::list","",15,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::markdown","",16,[[["formatter",3]],["result",6]]],[11,"fmt","markdown_composer::types::paragraph","",17,[[["formatter",3]],["result",6]]],[11,"builder","markdown_composer::types::image","",13,[[],["imagebuilder",3]]],[11,"builder","markdown_composer::types::link","",14,[[],["linkbuilder",3]]],[11,"builder","markdown_composer::types::list","",15,[[],["listbuilder",3]]]],"p":[[3,"ImageBuilder"],[3,"LinkBuilder"],[3,"ListBuilder"],[3,"CheckmarkItem"],[8,"Strikethrough"],[8,"MarkdownElement"],[8,"BlockQuote"],[8,"Bold"],[8,"CodeBlock"],[8,"Inline"],[8,"Italic"],[3,"Header"],[3,"HeaderLevel"],[3,"Image"],[3,"Link"],[3,"List"],[3,"Markdown"],[3,"Paragraph"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);