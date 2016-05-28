use conrod;

widget_ids! {
    CANVAS,
    HEADER,
    BODY,
    CHASER_COLUMN,
    EDITOR_COLUMN,
    TITLE,
    CHASER_TITLE,
    CONNECTED_BUTTON,
    EDITOR_BUTTON,
    ADD_CHASER_BUTTON,
    EDITOR_TITLE,
    EDITOR_INFO,
    EDITOR_TIME_SLIDER,
    EDITOR_CHASER_TITLE with 4000,
    EDITOR_CONTENT with 4000,
    BUTTON with 4000,
    CONTROL_CHASER_TITLE with 4000,
    EDITOR_SWITCH_SLIDER with 4000,
    EDITOR_SWITCH_BUTTON with 4000,
    EDITOR_SWITCH_TEXT with 4000,
    EDITOR_SWITCH_DROP_DOWNS with 4000,
    EDITOR_CURVE_STRING1,
    EDITOR_CURVE_STRING2
}

struct WidgetIDs {
    CANVAS: conrod::widget::id::Id,
    HEADER: conrod::widget::id::Id,
    BODY: conrod::widget::id::Id,
    CHASER_COLUMN: conrod::widget::id::Id,
    EDITOR_COLUMN: conrod::widget::id::Id,
    TITLE: conrod::widget::id::Id,
    CHASER_TITLE: conrod::widget::id::Id,
    CONNECTED_BUTTON: conrod::widget::id::Id,
    EDITOR_BUTTON: conrod::widget::id::Id,
    ADD_CHASER_BUTTON: conrod::widget::id::Id,
    EDITOR_TITLE: conrod::widget::id::Id,
    EDITOR_INFO: conrod::widget::id::Id,
    EDITOR_TIME_SLIDER: conrod::widget::id::Id,
    EDITOR_CHASER_TITLE: conrod::widget::id::Id,
    EDITOR_CONTENT: conrod::widget::id::Id,
    BUTTON: conrod::widget::id::Id,
    CONTROL_CHASER_TITLE: conrod::widget::id::Id,
    EDITOR_SWITCH_SLIDER: conrod::widget::id::Id,
    EDITOR_SWITCH_BUTTON: conrod::widget::id::Id,
    EDITOR_SWITCH_TEXT: conrod::widget::id::Id,
    EDITOR_SWITCH_DROP_DOWNS: conrod::widget::id::Id,
    EDITOR_CURVE_STRING1: conrod::widget::id::Id,
    EDITOR_CURVE_STRING2: conrod::widget::id::Id
}

impl WidgetIDs {
    pub fn generate() -> WidgetIDs {
        WidgetIDs {
            CANVAS: CANVAS,
            HEADER: HEADER,
            BODY: BODY,
            CHASER_COLUMN: CHASER_COLUMN,
            EDITOR_COLUMN: EDITOR_COLUMN,
            TITLE: TITLE,
            CHASER_TITLE: CHASER_TITLE,
            CONNECTED_BUTTON: CONNECTED_BUTTON,
            EDITOR_BUTTON: EDITOR_BUTTON,
            ADD_CHASER_BUTTON: ADD_CHASER_BUTTON,
            EDITOR_TITLE: EDITOR_TITLE,
            EDITOR_INFO: EDITOR_INFO,
            EDITOR_TIME_SLIDER: EDITOR_TIME_SLIDER,
            EDITOR_CHASER_TITLE: EDITOR_CHASER_TITLE,
            EDITOR_CONTENT: EDITOR_CONTENT,
            BUTTON: BUTTON,
            CONTROL_CHASER_TITLE: CONTROL_CHASER_TITLE,
            EDITOR_SWITCH_SLIDER: EDITOR_SWITCH_SLIDER,
            EDITOR_SWITCH_BUTTON: EDITOR_SWITCH_BUTTON,
            EDITOR_SWITCH_TEXT: EDITOR_SWITCH_TEXT,
            EDITOR_SWITCH_DROP_DOWNS: EDITOR_SWITCH_DROP_DOWNS,
            EDITOR_CURVE_STRING1: EDITOR_CURVE_STRING1,
            EDITOR_CURVE_STRING2: EDITOR_CURVE_STRING2
        }
    }
}
