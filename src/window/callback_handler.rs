pub type OnEventCallback<'a, WindowData> =
    dyn 'a + FnMut(&mut WindowData, &sdl2::event::Event) -> super::Result<()>;
pub type OnFrameCallback<'a, WindowData> = dyn 'a + FnMut(&mut WindowData) -> super::Result<()>;

pub struct CallbackHandler<'a, WindowData> {
    on_event_cbs: Vec<Box<OnEventCallback<'a, WindowData>>>,
    on_frame_cbs: Vec<Box<OnFrameCallback<'a, WindowData>>>,
}

impl<'a, WindowData> Default for CallbackHandler<'a, WindowData> {
    fn default() -> Self {
        Self {
            on_event_cbs: Default::default(),
            on_frame_cbs: Default::default(),
        }
    }
}

impl<'a, WindowData> CallbackHandler<'a, WindowData> {
    fn add_event_cb_box(&mut self, callback: Box<OnEventCallback<'a, WindowData>>) {
        self.on_event_cbs.push(callback);
    }
    fn add_frame_cb_box(&mut self, callback: Box<OnFrameCallback<'a, WindowData>>) {
        self.on_frame_cbs.push(callback);
    }

    pub fn add_event_cb_data(
        &mut self,
        callback: impl 'a + FnMut(&mut WindowData, &sdl2::event::Event) -> super::Result<()>,
    ) {
        self.add_event_cb_box(Box::new(callback))
    }
    pub fn add_frame_cb_data(
        &mut self,
        callback: impl 'a + FnMut(&mut WindowData) -> super::Result<()>,
    ) {
        self.add_frame_cb_box(Box::new(callback))
    }

    #[allow(dead_code)]
    pub fn add_event_cb(
        &mut self,
        mut callback: impl 'a + FnMut(&sdl2::event::Event) -> super::Result<()>,
    ) {
        self.add_event_cb_data(move |_, event| callback(event))
    }
    #[allow(dead_code)]
    pub fn add_frame_cb(&mut self, mut callback: impl 'a + FnMut() -> super::Result<()>) {
        self.add_frame_cb_data(move |_| callback())
    }

    pub(super) fn call_on_event_cbs(
        &mut self,
        shared_data: &mut WindowData,
        event: &sdl2::event::Event,
    ) -> super::Result<()> {
        for cb in &mut self.on_event_cbs {
            cb(shared_data, event)?;
        }
        Ok(())
    }
    pub(super) fn call_on_frame_cbs(&mut self, shared_data: &mut WindowData) -> super::Result<()> {
        for cb in &mut self.on_frame_cbs {
            cb(shared_data)?;
        }
        Ok(())
    }
}
