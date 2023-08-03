import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const paymentRouter = router({
  send: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
        amount: z.string(),
        sender: z.string(),
        receiver: z.string(),
        balance: z.string(),
        description: z.string(),
      }),
    )
    .mutation(async (opts) => {
      try {
        const method = `${opts.input.method}`
        const cookie = `${opts.input.cookie}`
        const receiver_name = `${opts.input.receiver.split(" ")[1]}`
        const receiver_id = `${opts.input.receiver.split(" ")[0]}`
        const id = opts.input.id
        console.log(+receiver_id, receiver_name)
        const url = "http://localhost:8080/api/rpc"
        const headers = {
          Cookie: cookie
        }
        let payments = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id,
            method,
            params: {
              data: {
                amount: `${opts.input.amount}`,
                sender: `${opts.input.sender}`,
                receiver: `${receiver_name}`,
                description: `${opts.input.description}`,
              }
            }
          }
        });

        const balance = +opts.input.balance
        const amount = +opts.input.amount
        const new_acc_balance = balance - +amount
        let update_account_data_response = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id: 1,
            method: "update_account",
            params: {
              id: id,
              data: {
                balance: `${new_acc_balance}`
              }
            }
          }
        })
        let get_receiver_account_data_response = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id: id,
            method: "get_account",
            params: {
              id: +receiver_id
            }
          }
        })
        const receiver_balance = get_receiver_account_data_response.data.result.data.balance
      const rec_balance = +receiver_balance
      const new_rec_balance = rec_balance + +opts.input.amount
        let update_receiver_account_data_response = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id: 1,
            method: "update_account",
            params: {
              id: +receiver_id,
              data: {
                balance: `${new_rec_balance}`
              }
            }
          }
        })
        return {
          payments: payments.data,
          user_account: update_account_data_response.data.result.data,
          receiver_account: update_receiver_account_data_response.data.result.data,
        };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
  list: procedure
    .input(
      z.object({
        cookie: z.string(),
        method: z.string(),
        id: z.number(),
      }),
    )
    .query(async (opts) => {
      try {
        const method = `${opts.input.method}`
        const cookie = `${opts.input.cookie}`
        const id = opts.input.id
        const url = "http://localhost:8080/api/rpc"
        const headers = {
          Cookie: cookie
        }
        let payments = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id,
            method,
          }
        });

        return {
          payments: payments.data,
        };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
});


