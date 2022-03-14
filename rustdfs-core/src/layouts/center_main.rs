use crate::models::Tag;
use crate::models::Window;
use crate::models::Workspace;

pub fn update(workspace: &Workspace, tag: &Tag, windows: &mut [&mut Window]) {
    let window_count = windows.len();

    if window_count == 0 {
        return;
    }

    let column_count = match window_count {
        1 | 2 => window_count,
        _ => 3,
    };
    let workspace_width = workspace.width_limited(column_count);
    let workspace_x = workspace.x_limited(column_count);

    let primary_width = match window_count {
        1 => workspace_width,
        _ => ((workspace_width as f32 / 100.0) * tag.main_width_percentage()).floor() as i32,
    };
    let secondary_width = match window_count {
        1 => 0,
        2 => workspace_width - primary_width,
        _ => ((workspace_width - primary_width) as f32 /  2.0).floor() as i32,
    };
    let (primary_x, secondary_x, stack_x) = match window_count {
        1 => (workspace_x, 0, 0),
        2 => {
            let (px, sx);
            if tag.flipped_horizontal {
                px = workspace_x;
                sx = workspace_x + secondary_width;
            } else {
                px = workspace_x + secondary_width;
                sx = workspace_x;
            }
            (px, sx, 0)
        }
        _ => {
           let px = workspace_x + secondary_width; 
           let (sx, stx);
           if tag.flipped_horizontal {
               sx = workspace_x + primary_width + secondary_width;
               stx = workspace_x;
           } else {
               sx = workspace_x;
               stx = workspace_x + primary_width + secondary_width;
           }
           (px, sx, stx)
        }
    };

    let mut iter = windows.iter_mut();
    {
        if let Some(first) = iter.next() {
            first.set_height(workspace.height());
            first.set_width(primary_width);
            first.set_x(primary_x);
            first.set_y(workspace.y());
        }
    }
     
    {
        if let Some(second) = iter.next() {
            second.set_height(workspace.height());
            second.set_width(secondary_width);
            second.set_x(secondary_x);
            second.set_y(workspace.y());
        }
    }
    
    if window_count > 2 {
        let height_f = workspace.height() as f32 / (window_count - 2) as f32;
        let height = height_f.floor() as i32;
        let mut y = 0;

        for w in iter {
            w.set_height(height);
            w.set_width(secondary_width);
            w.set_x(stack_x);
            w.set_y(workspace.y() + y);
            y += height;
        }
    }
}
