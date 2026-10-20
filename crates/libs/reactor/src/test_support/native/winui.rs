use super::*;

impl WinUiAdapter {
    /// Reads the WinUI system-cursor discriminant for native integration tests.
    pub fn pointer_cursor_shape(&self, object: ObjectId) -> Result<i32, WinUiError> {
        Ok(self
            .ui_element(object)?
            .cast::<native::IUIElementProtected>()?
            .ProtectedCursor()?
            .cast::<native::InputSystemCursor>()?
            .CursorShape()?
            .0)
    }

    pub(crate) fn realize_virtual_item(
        &self,
        object: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        let index = i32::try_from(index).map_err(|_| WinUiError::IndexOverflow(index))?;
        match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                repeater.GetOrCreateElement(index)?;
                Ok(())
            }
            Some(_) => Err(WinUiError::InvalidObject(object)),
            None => Err(WinUiError::MissingObject(object)),
        }
    }

    pub(crate) fn virtual_shell_counts(&self) -> (usize, usize) {
        self.virtual_items
            .values()
            .fold((0, 0), |(live, retired), items| {
                let pool = items.shells.pool.borrow();
                (live + pool.shells.len(), retired + pool.retired.len())
            })
    }
}
