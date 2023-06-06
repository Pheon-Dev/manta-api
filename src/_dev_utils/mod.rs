mod dev_db;

use crate::ctx::Ctx;
use crate::model::send_req::{SendReq, SendReqBmc, SendReqForCreate};
use crate::model::{self, ModelManager};

use tokio::sync::OnceCell;
use tracing::info;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init_dev()", "FOR-DEV-ONLY");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}

pub async fn seed_tasks(
    ctx: &Ctx,
    mm: &ModelManager,
    amounts: &[&str],
    currencies: &[&str],
    receivers: &[&str],
    descriptions: &[&str],
) -> model::Result<Vec<SendReq>> {
    let mut send_reqs = Vec::new();

    for amount in amounts {
        for currency in currencies {
            for receiver in receivers {
                for description in descriptions {
                    let id = SendReqBmc::create(
                        ctx,
                        mm,
                        SendReqForCreate {
                            amount: amount.to_string(),
                            currency: currency.to_string(),
                            receiver: receiver.to_string(),
                            description: description.to_string(),
                        },
                    )
                    .await?;
                    let send_req = SendReqBmc::get(ctx, mm, id).await?;
                    send_reqs.push(send_req);
                }
            }
        }
    }

    Ok(send_reqs)
}
