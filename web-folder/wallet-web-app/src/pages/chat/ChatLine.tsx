import { Group, Paper, Loader, Text, Divider, Badge, Card } from '@mantine/core'
import Balancer from 'react-wrap-balancer'
import { useMantaStore } from '../_app';

// wrap Balancer to remove type errors :( - @TODO - fix this ugly hack
const BalancerWrapper = (props: any) => <Balancer {...props} />

type ChatGPTAgent = 'user' | 'system' | 'assistant'

export interface ChatGPTMessage {
  role: ChatGPTAgent
  content: string
}

// loading placeholder animation for the chat line
export const LoadingChatLine = () => (
  <Loader variant="bars" my="xs" size="sm" />
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
  const id = useMantaStore((state) => state.id)
  const formatteMessage = convertNewLines(content)

  return (
    <Group
      position={
        role != 'assistant' ? 'right' : 'left'
      }
      my="lg"
    >
      <BalancerWrapper>
        <Paper shadow="lg" p="xl" radius="lg" withBorder color={role == 'assistant' ? 'green' : 'blue'}>
          <Group>
            <Text>
              {formatteMessage}
            </Text>
          </Group>
        </Paper>
        <Badge variant="gradient" gradient={{ from: 'indigo', to: 'cyan', deg: 45 }} my="xs">
          {role == 'assistant' ? 'AI Assistant' : `You: ${id}`}
        </Badge>
      <Divider my="xs" variant="dashed" />
      </BalancerWrapper>
    </Group>
  )
}
