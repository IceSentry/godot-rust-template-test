#[macro_export]
macro_rules! get_node {
    ($node:ident, $target:ty, $path:expr) => {{
        unsafe {
            $node
                .get_node_as::<$target>($path)
                .unwrap_or_else(|| panic!("{} not found", $path))
        }
    }};
}

#[macro_export]
macro_rules! get_node_as_instance {
    ($node:ident, $target:ty, $path:expr) => {{
        unsafe {
            $node
                .get_node_as_instance::<$target>($path)
                .unwrap_or_else(|| panic!("{} not found", $path))
        }
    }};
}
