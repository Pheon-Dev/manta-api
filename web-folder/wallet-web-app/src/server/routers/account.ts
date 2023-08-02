import axios from 'axios';
import { z } from 'zod';
import { procedure, router } from '../trpc';

export const accountRouter = router({
  create: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
        username: z.string(),
        balance: z.string(),
        email: z.string(),
        aid: z.string(),
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
                username: `${opts.input.username}`,
                balance: `${opts.input.balance}`,
                email: `${opts.input.email}`,
                aid: `${opts.input.aid}`,
              }
            }
          }
        });

        return {
          payments: payments.data,
        };
      } catch (error) {
        return { message: error }

      }
    }),
  withdraw: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
        amount: z.number(),
        card_id: z.number(),
        acc_id: z.number(),
        username: z.string(),
        balance: z.string(),
      }),
    )
    .mutation(async (opts) => {
      try {
        const get_card_method = "get_card"
        const cookie = `${opts.input.cookie}`
        const card_id = opts.input.card_id
        const acc_id = opts.input.acc_id
        const username = `${opts.input.username}`
        const balance = `${opts.input.balance}`
        const amount = opts.input.amount
        const id = opts.input.id
        const url = "http://localhost:8080/api/rpc"
        const headers = {
          Cookie: cookie
        }
        let get_card_data_response = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id: 1,
            method: get_card_method,
            params: {
              id: card_id
            }
          }
        });

        const card = get_card_data_response.data
        if (card) {
          const current_balance = +card.result.data.cbalance
          const new_balance = current_balance + amount
          const acc_balance = +balance
          if (acc_balance < 1) return { message: "Insufficient balance" }
          const new_acc_balance = acc_balance - amount
          if (new_acc_balance < 1) return { message: "Insufficient balance" }
          let update_account_data_response = await axios.request({
            method: "POST",
            url,
            headers,
            data: {
              id: 1,
              method: "update_account",
              params: {
                id: acc_id,
                data: {
                  balance: `${new_acc_balance}`
                }
              }
            }
          })
          let update_card_data_response = await axios.request({
            method: "POST",
            url,
            headers,
            data: {
              id: 1,
              method: "update_card",
              params: {
                id: card_id,
                data: {
                  cbalance: `${new_balance}`
                }
              }
            }
          })
          return {
            update_card_data_response: update_card_data_response.data,
          };
        };
      } catch (error) {
        return { message: error }

      }
    }),
  deposit: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
        amount: z.number(),
        card_id: z.number(),
        acc_id: z.number(),
        username: z.string(),
        balance: z.string(),
      }),
    )
    .mutation(async (opts) => {
      try {
        const get_card_method = "get_card"
        const cookie = `${opts.input.cookie}`
        const card_id = opts.input.card_id
        const acc_id = opts.input.acc_id
        const username = `${opts.input.username}`
        const balance = `${opts.input.balance}`
        const amount = opts.input.amount
        const id = opts.input.id
        const url = "http://localhost:8080/api/rpc"
        const headers = {
          Cookie: cookie
        }

        let get_card_data_response = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id: 1,
            method: get_card_method,
            params: {
              id: card_id
            }
          }
        });

        const card = get_card_data_response.data
        if (card) {
          const current_balance = +card.result.data.cbalance
          if (current_balance < 1) return { message: "Insufficient balance" }
          const new_balance = current_balance - amount
          if (new_balance < 1) return { message: "Insufficient balance" }
          const acc_balance = +balance
          const new_acc_balance = acc_balance + amount

          let update_account_data_response = await axios.request({
            method: "POST",
            url,
            headers,
            data: {
              id: 1,
              method: "update_account",
              params: {
                id: acc_id,
                data: {
                  balance: `${new_acc_balance}`
                }
              }
            }
          })

          let update_card_data_response = await axios.request({
            method: "POST",
            url,
            headers,
            data: {
              id: 1,
              method: "update_card",
              params: {
                id: card_id,
                data: {
                  cbalance: `${new_balance}`
                }
              }
            }
          })
          return {
            update_card_data_response: update_card_data_response.data,
          };
        }
        if (!card)
          return {
            message: "Card was not loaded"
          };
        return {
          get_card_data_response: get_card_data_response.data,
        };
      } catch (error) {
        return { message: error }

      }
    }),
  list: procedure
    .input(
      z.object({
        id: z.number(),
        method: z.string(),
        cookie: z.string(),
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

        let data = await axios.request({
          method: "POST",
          url,
          headers,
          data: {
            id,
            method,
          }
        });

        return {
          data: data.data,
        };
      } catch (error) {
        return {
          error: error,
          message: "Internal Server Error check connection",
        }
      }
    }),
});
