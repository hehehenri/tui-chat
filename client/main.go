package main

import (
	"fmt"
	"log"
	"strings"

	"github.com/charmbracelet/bubbles/textarea"
	"github.com/charmbracelet/bubbles/viewport"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

func main() {
	p := tea.NewProgram(initialModel())

	if _, err := p.Run(); err != nil {
		log.Fatal(err)
	}
}

type ErrorMessage error

type Model struct {
	view     viewport.Model
	input    textarea.Model
	messages []string
	style    lipgloss.Style
	err      error
}

func initialModel() Model {
	input := textarea.New()
	input.Placeholder = "Send a message..."
	input.Focus()

	input.Prompt = "┃ "
	input.CharLimit = 280

	input.SetWidth(30)
	input.SetHeight(3)

	// Remove cursor line styling
	input.FocusedStyle.CursorLine = lipgloss.NewStyle()

	input.ShowLineNumbers = false

	view := viewport.New(30, 5)
	view.SetContent(`Welcome to the chat room!
Type a message and press Enter to send.`)

	input.KeyMap.InsertNewline.SetEnabled(false)

	return Model{
		input:    input,
		messages: []string{},
		view:     view,
		style:    lipgloss.NewStyle().Foreground(lipgloss.Color("5")),
		err:      nil,
	}
}

func (m Model) Init() tea.Cmd {
	return textarea.Blink
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var (
		tiCmd tea.Cmd
		vpCmd tea.Cmd
	)

	m.input, tiCmd = m.input.Update(msg)
	m.view, vpCmd = m.view.Update(msg)

	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyCtrlC, tea.KeyEsc:
			fmt.Println(m.input.Value())
			return m, tea.Quit
		case tea.KeyEnter:
			m.messages = append(m.messages, m.style.Render("You: ")+m.input.Value())
			m.view.SetContent(strings.Join(m.messages, "\n"))
			m.input.Reset()
			m.view.GotoBottom()
		}

	// We handle errors just like any other message
	case ErrorMessage:
		m.err = msg
		return m, nil
	}

	return m, tea.Batch(tiCmd, vpCmd)
}

func (m Model) View() string {
	return fmt.Sprintf(
		"%s\n\n%s",
		m.view.View(),
		m.input.View(),
	) + "\n\n"
}
