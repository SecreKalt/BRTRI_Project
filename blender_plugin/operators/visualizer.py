import bpy
import numpy as np
import bmesh
from ..utils.optimizers import optimize_point_cloud
import open3d as o3d
from mathutils import Matrix
import gpu
from gpu_extras.batch import batch_for_shader

class BRTRI_OT_UpdateMesh(bpy.types.Operator):
    bl_idname = "brtri.update_mesh"
    bl_label = "Update Mesh Visualization"
    
    @classmethod
    def update_point_cloud(cls, data):
        try:
            points = np.array(data['points'], dtype=np.float32)
            
            # Use GPU for point cloud processing if available
            if hasattr(gpu, "state"):
                cls._update_gpu_visualization(points)
            else:
                optimized_points = optimize_point_cloud(points)
                cls._update_blender_mesh(optimized_points)
                
        except Exception as e:
            from ..operators.error_handler import BRTRI_ErrorHandler
            BRTRI_ErrorHandler.log_error(f"Visualization error: {str(e)}")
    
    @staticmethod
    def _update_gpu_visualization(points):
        shader = gpu.shader.from_builtin('3D_UNIFORM_COLOR')
        batch = batch_for_shader(shader, 'POINTS', {"pos": points})
        
        def draw():
            shader.bind()
            shader.uniform_float("color", (1, 1, 1, 1))
            batch.draw(shader)
        
        gpu.state.point_size_set(5)
        bpy.types.SpaceView3D.draw_handler_add(draw, (), 'WINDOW', 'POST_VIEW')
    
    @staticmethod
    def _update_blender_mesh(points):
        mesh_name = "BRTRI_Scan"
        if mesh_name in bpy.data.meshes:
            mesh = bpy.data.meshes[mesh_name]
            bm = bmesh.new()
            bm.from_mesh(mesh)
            bm.clear()
        else:
            mesh = bpy.data.meshes.new(mesh_name)
            obj = bpy.data.objects.new(mesh_name, mesh)
            bpy.context.scene.collection.objects.link(obj)
            bm = bmesh.new()
            
        # Use BMesh for faster updates
        for p in points:
            bm.verts.new(p)
        
        bm.to_mesh(mesh)
        bm.free()
        mesh.update()
    
    def execute(self, context):
        return {'FINISHED'}
