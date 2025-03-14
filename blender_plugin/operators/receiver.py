import bpy
from ..communication.bridge_client import BRTRIBridgeClient
from ..operators.error_handler import BRTRI_ErrorHandler

bridge_client = None

class BRTRI_OT_StartReceiver(bpy.types.Operator):
    bl_idname = "brtri.start_receiver"
    bl_label = "Start LiDAR Receiver"
    
    def execute(self, context):
        global bridge_client
        try:
            bridge_client = BRTRIBridgeClient()
            if bridge_client.start():
                context.scene.brtri_connection_active = True
                return {'FINISHED'}
            return {'CANCELLED'}
        except Exception as e:
            BRTRI_ErrorHandler.log_error(str(e))
            return {'CANCELLED'}

class BRTRI_OT_StopReceiver(bpy.types.Operator):
    bl_idname = "brtri.stop_receiver"
    bl_label = "Stop LiDAR Receiver"
    
    def execute(self, context):
        global bridge_client
        try:
            if bridge_client:
                bridge_client.stop()
                context.scene.brtri_connection_active = False
            return {'FINISHED'}
        except Exception as e:
            BRTRI_ErrorHandler.log_error(str(e))
            return {'CANCELLED'}
