package main

import (
	"fmt"
	"log"
	"net"

	"github.com/charmbracelet/bubbles/textarea"
	"github.com/charmbracelet/bubbles/viewport"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/google/uuid"
)

func main() {
	p := tea.NewProgram(initialModel())

	if _, err := p.Run(); err != nil {
		log.Fatal(err)
	}
}

type Model struct {
	client   Peer
	peers    []Peer
	messages []Message
	view     viewport.Model
	input    textarea.Model
}

func (m Model) sendMessage(content string) {
	message := Message{
		sender:  m.client,
		content: content,
	}

	m.messages = append(m.messages, message)
}

func (Model) Init() tea.Cmd {
	return textarea.Blink
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var inputCmd tea.Cmd
	var viewCmd tea.Cmd

	m.input, inputCmd = m.input.Update(msg)
	m.view, viewCmd = m.view.Update(msg)

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyCtrlC, tea.KeyEsc:
			fmt.Println(m.input.Value())
			return m, tea.Quit
		case tea.KeyEnter:
			m.sendMessage(m.input.View())

			content := ""
			for _, message := range m.messages {
				content += message.view() + "\n"
			}

			m.view.SetContent(content)
			m.view.GotoBottom()
			m.input.Reset()
		}
	}

	return m, tea.Batch(inputCmd, viewCmd)
}

func (m Model) View() string {
	return fmt.Sprintf(
		"%s\n\n%s",
		m.view.View(),
		m.input.View(),
	) + "\n\n"
}

func initialModel() Model {
	input := textarea.New()
	input.Placeholder = "Send a message..."
	input.Focus()

	input.Prompt = "┃ "
	input.CharLimit = 280

	input.SetWidth(30)
	input.SetHeight(3)

	input.ShowLineNumbers = false

	view := viewport.New(30, 5)
	view.SetContent(`Welcome to the chat!
Type a message and press Enter to send.`)

	input.KeyMap.InsertNewline.SetEnabled(false)

	return Model{
		client:   Peer{},
		peers:    []Peer{},
		messages: []Message{},
		view:     view,
		input:    input,
	}
}

type Message struct {
	sender  Peer
	content string
}

func (m Message) view() string {
	return fmt.Sprint("%s: %s", m.sender.id, m.content)
}

type Peer struct {
	id   uuid.UUID
	addr net.UDPAddr
}
