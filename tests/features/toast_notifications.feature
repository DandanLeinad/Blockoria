Feature: Toast Notifications

  @stack
  Scenario: Multiple toasts stack vertically with gap
    Given ToastProvider is mounted
    When 3 toasts are created rapidly via toast.success()
    Then container renders 3 toasts stacked vertically with 8px gap
    And each toast has correct type styling

  @stack-limit
  Scenario: Max 5 toasts visible, oldest removed (FIFO)
    Given ToastProvider with maxVisible: 5
    When 7 toasts are created rapidly
    Then only 5 toasts are in DOM
    And the first 2 created are removed

  @animation-exit
  Scenario: Toast animates out before unmount
    Given a toast is visible
    When dismiss() is called OR duration expires
    Then toast runs slide-out+fade animation (200ms)
    And only after animation completes it is removed from DOM

  @progress-bar
  Scenario: Progress bar animates and pauses on hover
    Given a toast with duration: 4000
    When toast mounts
    Then progress bar animates width 100% → 0% over 4000ms
    When user hovers toast
    Then progress bar animation pauses
    When user stops hovering
    Then progress bar resumes from paused position

  @hover-pause
  Scenario: Auto-dismiss pauses on hover/focus
    Given a toast with duration: 2000
    When user hovers toast at 1000ms
    And waits 2000ms
    And stops hovering
    Then toast dismisses at ~3000ms total (not 2000ms)

  @action-button
  Scenario: Action button executes callback and dismisses
    Given toast with action: { label: "Desfazer", onClick: fn }
    When user clicks action button
    Then fn is called
    And toast dismisses with animation

  @rich-content
  Scenario: Rich content renders JSX instead of message string
    Given toast with richContent: <strong>Bold</strong> message
    When toast renders
    Then JSX is rendered
    And message string used as aria-label fallback

  @persist
  Scenario: Persist toast has no progress bar and no auto-dismiss
    Given toast with persist: true
    When toast mounts
    Then no progress bar rendered
    And toast remains until dismiss() or action clicked

  @positions
  Scenario Outline: Toast container renders at configured position
    Given ToastProvider with position: "<position>"
    When toast is created
    Then container has correct fixed positioning
    Examples:
      | position      |
      | top-right     |
      | top-left      |
      | bottom-right  |
      | bottom-left   |
      | top-center    |
      | bottom-center |

  @a11y
  Scenario: Screen reader announces toast correctly
    Given error toast with message "Erro ao salvar"
    When toast mounts
    Then role="alert" and aria-live="assertive"
    And aria-atomic="true"
    And screen reader announces "Erro ao salvar"

  Scenario: Info toast uses polite live region
    Given info toast
    When toast mounts
    Then role="status" and aria-live="polite"

  @keyboard
  Scenario: Escape key dismisses focused toast
    Given a toast is visible and focused
    When user presses Escape
    Then toast dismisses with animation

  Scenario: Tab navigates between close and action buttons
    Given toast with action button
    When user presses Tab
    Then focus moves: close → action → close (cycle)
