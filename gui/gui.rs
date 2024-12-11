use eframe::run_native;

struct Headlines;

impl App for Headlines{
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>){
        CentralPanel::default().show(ctx, add_contents: |ui&mut ui| {
            ui.label("article text");
        });
    

    fn name(&self) -> &str {
        "Headlines"
    }
}


fn main(){
    
    run_native(app, native_options);


        

}



