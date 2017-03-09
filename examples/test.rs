extern crate sanngaa;

use sanngaa::traits::*;

fn main() {
    // <?xml version="1.0" encoding="UTF-8" ?>
    let input = r#"
    <root>
	<request>
		<field name="name" />
		<field name="name2" />
		<nosee>
			<see:field name="nosee" />
			<see:description><![CDATA[
Creates an unmapped window as child of the specified `parent` window. A
CreateNotify event will be generated. The new window is placed on top in the
stacking order with respect to siblings.

The coordinate system has the X axis horizontal and the Y axis vertical with
the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
of pixels, and coincide with pixel centers. Each window and pixmap has its own
coordinate system. For a window, the origin is inside the border at the inside,
upper-left corner.

The created window is not yet displayed (mapped), call `xcb_map_window` to
display it.

The created window will initially use the same cursor as its parent.
			]]></see:description>
		</see:nosee>
	</request>
</root>"#;

    let dom = sanngaa::parse_xml().one(input);

    println!("{}", dom.to_string());

    for field in dom.select("request > field").unwrap() {
        println!("{:?}", field);
    }
}
