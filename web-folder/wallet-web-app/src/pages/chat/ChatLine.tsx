import { Group, Text } from '@mantine/core'
import Balancer from 'react-wrap-balancer'

// wrap Balancer to remove type errors :( - @TODO - fix this ugly hack
const BalancerWrapper = (props: any) => <Balancer {...props} />

type ChatGPTAgent = 'user' | 'system' | 'assistant'

export interface ChatGPTMessage {
  role: ChatGPTAgent
  content: string
}

// loading placeholder animation for the chat line
export const LoadingChatLine = () => (
  <h1>AI:</h1>
)

// util helper to convert new lines to <br /> tags
const convertNewLines = (text: string) =>
  text.split('\n').map((line, i) => (
    <span key={i}>
      {line}
      <br />
    </span>
  ))

export function ChatLine({ role = 'assistant', content }: ChatGPTMessage) {
  if (!content) {
    return null
  }
  const formatteMessage = convertNewLines(content)

  return (
    <Group
      position={
        role != 'assistant' ? 'right' : 'left'
      }
    >
      <BalancerWrapper>
              <p>
                <a href="#" className="hover:underline">
                  {role == 'assistant' ? 'AI' : 'You'}
                </a>
              </p>
              <Text color={role == 'assistant' ? 'green' : 'blue' }>
                {formatteMessage}
              </Text>
      </BalancerWrapper>
    </Group>
  )
}
