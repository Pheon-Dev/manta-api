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
            params: {
              data: {
                amount: `${opts.input.amount}`,
                sender: `${opts.input.sender}`,
                receiver: `${opts.input.receiver}`,
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


