import bpy
import sys
import os
from pathlib import Path
from .operators.receiver import BRTRI_OT_StartReceiver, BRTRI_OT_StopReceiver
from .operators.visualizer import BRTRI_OT_UpdateMesh
from .operators.error_handler import BRTRI_ErrorHandler

bl_info = {
    "name": "BRTRI LiDAR Integration",
    "description": "Real-time LiDAR data visualization with ZMQ integration",
    "author": "BRTRI Project",
    "version": (1, 0, 0),
    "blender": (3, 3, 0),
    "location": "View3D > Sidebar > BRTRI",
    "warning": "",
    "category": "3D View",
}


# Add plugin directory to system path
plugin_dir = Path(os.path.dirname(os.path.realpath(__file__)))
if str(plugin_dir) not in sys.path:
    sys.path.append(str(plugin_dir))

# Import plugin modules
from .operators import receiver, visualizer, error_handler

# Panel class for UI
class BRTRI_PT_MainPanel(bpy.types.Panel):
    bl_label = "BRTRI LiDAR Control"
    bl_idname = "BRTRI_PT_main_panel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = 'BRTRI'

    def draw(self, context):
        layout = self.layout
        
        # Connection status
        box = layout.box()
        box.label(text="Connection Status")
        row = box.row()
        row.operator("brtri.start_receiver", text="Start Connection")
        row.operator("brtri.stop_receiver", text="Stop Connection")
        
        # Visualization settings
        box = layout.box()
        box.label(text="Visualization")
        box.operator("brtri.update_mesh", text="Update Mesh")
        
        # Display error status if any
        if hasattr(context.scene, "brtri_error_message"):
            box = layout.box()
            box.label(text="Error Status")
            box.label(text=context.scene.brtri_error_message)

# Classes to register
classes = (
    BRTRI_OT_StartReceiver,
    BRTRI_OT_StopReceiver,
    BRTRI_OT_UpdateMesh,
    BRTRI_PT_MainPanel,
)

# Custom properties
def register_properties():
    bpy.types.Scene.brtri_error_message = bpy.props.StringProperty(
        name="Error Message",
        default=""
    )
    bpy.types.Scene.brtri_connection_active = bpy.props.BoolProperty(
        name="Connection Active",
        default=False
    )

def unregister_properties():
    del bpy.types.Scene.brtri_error_message
    del bpy.types.Scene.brtri_connection_active

def register():
    # Register classes
    for cls in classes:
        bpy.utils.register_class(cls)
    
    # Register properties
    register_properties()
    
    # Initialize error handler
    BRTRI_ErrorHandler.initialize()

def unregister():
    # Unregister classes
    for cls in reversed(classes):
        bpy.utils.unregister_class(cls)
    
    # Unregister properties
    unregister_properties()
    
    # Cleanup error handler
    BRTRI_ErrorHandler.cleanup()

if __name__ == "__main__":
    register()
