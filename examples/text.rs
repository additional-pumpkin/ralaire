use ralaire::app::App;
use ralaire::widget::{container, empty, text, Widget};
use ralaire_core::{Command, Padding};
#[derive(Debug, Clone)]
enum Message {}

#[derive(Clone, PartialEq)]
struct Text;

impl App for Text {
    type Message = Message;

    fn new() -> Self {
        Text
    }

    fn title(&self) -> impl Into<String> {
        "Examples - Text"
    }

    fn header(&self) -> impl Widget<Self::Message> + 'static {
        empty()
    }

    fn view(&self) -> impl Widget<Self::Message> + 'static {
        container(text(
"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam at turpis orci. Cras non iaculis sem. Donec at pulvinar erat. Ut congue enim felis, vel suscipit est efficitur nec. Sed at risus sed nibh elementum fermentum sed nec urna. Praesent ullamcorper malesuada tellus, in imperdiet ipsum finibus vitae. Ut consequat varius neque at aliquam. Ut in velit volutpat, eleifend neque quis, semper dolor. Fusce mattis, libero vel interdum consequat, neque metus pulvinar dolor, sed ultricies velit metus ac sapien. Curabitur varius magna et volutpat consectetur. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Quisque varius justo vitae odio tempor, sit amet aliquam enim blandit. Donec aliquam dolor arcu, nec posuere neque tristique at. Fusce iaculis eget justo ac tempor.

Curabitur volutpat tincidunt dolor, ac sagittis dolor laoreet nec. Morbi faucibus bibendum ornare. Suspendisse magna orci, laoreet ac sagittis varius, fringilla at lorem. Sed posuere aliquet velit, commodo lobortis ante bibendum in. Suspendisse sed sem nibh. Duis vitae pharetra erat. Interdum et malesuada fames ac ante ipsum primis in faucibus. Integer sodales tincidunt lectus, id ultricies magna tempor sed. Phasellus porta tellus ac scelerisque maximus. Pellentesque blandit arcu dui, id consequat purus lacinia in.       

Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Nulla interdum, justo eget convallis volutpat, ligula enim commodo ipsum, sit amet tempor eros diam nec erat. Nulla quis justo sed orci aliquam accumsan et sit amet dui. Suspendisse hendrerit nisi urna, placerat euismod massa ornare et. Vestibulum est dui, vestibulum ut dignissim sed, condimentum quis nisi. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Fusce et neque fermentum, auctor orci quis, congue nisl. Nulla facilisis hendrerit nibh. Morbi nec placerat risus, sed maximus ligula.

Quisque laoreet ligula ut viverra eleifend. Donec sapien quam, vehicula ut pretium nec, dictum quis ipsum. Nulla vulputate ipsum id justo elementum tincidunt. Phasellus imperdiet porttitor lorem, sit amet cursus erat sollicitudin a. Sed nec convallis mi. Nullam rhoncus vulputate odio vel placerat. Morbi in pulvinar ipsum. Nullam nec augue elementum nibh rhoncus pharetra ut luctus enim. Fusce vulputate ante ipsum, eu faucibus ligula convallis ac. Vestibulum ut lacus id orci venenatis sollicitudin. Vestibulum dapibus ligula nec pretium aliquet. Aliquam dapibus, arcu nec aliquam ullamcorper, velit magna venenatis turpis, at efficitur risus nunc quis odio. Donec egestas lacus at odio cursus, sed luctus turpis sagittis. Nulla fermentum luctus tincidunt. Proin tristique, elit at tempus iaculis, orci lorem viverra enim, ac consequat sem lectus in libero. Morbi placerat malesuada turpis a luctus.

Etiam posuere egestas congue. In et viverra nulla. Vestibulum nec purus convallis, luctus nunc ut, bibendum nibh. Donec nisl turpis, ultricies eget vulputate a, vestibulum ac nisl. Sed in eros mauris. Donec vitae augue in est ullamcorper commodo at ac lectus. Aliquam blandit nibh nec nunc bibendum euismod. Ut quis malesuada magna. Aliquam tincidunt ornare quam sit amet commodo. Suspendisse gravida sed nunc efficitur condimentum. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Cras sit amet pretium felis.
")).pad(Padding::from([0.,100.]))
    }

    fn update(&mut self, message: Self::Message) -> Vec<Command<Self::Message>> {
        match message {}
    }
}
fn main() -> ralaire::Result {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Text::run()
}
