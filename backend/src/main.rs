use moon::*;

async fn frontend() -> Frontend {
    Frontend::new().title("Meditation timer").append_to_head(
        "
        <link rel='manifest' href='/_api/public/meditationtimer.webmanifest'>

        <style>
            body {
              background-image: url('/_api/public/background-leaves.png');
              background-repeat: no-repeat;
              background-attachment: fixed;
              background-size: cover;
            }
        </style>

        <script>
            if ('serviceWorker' in navigator) {
              navigator.serviceWorker.register('_api/public/sw.js', {}).then(function(reg) {

                if(reg.installing) {
                  console.log('Service worker installing');
                } else if(reg.waiting) {
                  console.log('Service worker installed');
                } else if(reg.active) {
                  console.log('Service worker active');
                }

              }).catch(function(error) {
                // registration failed
                console.log('Registration failed with ' + error);
              });
            }
        </script>",
    )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
