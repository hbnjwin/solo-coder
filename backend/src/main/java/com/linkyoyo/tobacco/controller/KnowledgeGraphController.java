package com.linkyoyo.tobacco.controller;

import com.linkyoyo.tobacco.dto.StandardGraphDto;
import com.linkyoyo.tobacco.dto.StandardReferenceResponse;
import com.linkyoyo.tobacco.entity.neo4j.EnhancedStandardNode;
import com.linkyoyo.tobacco.entity.neo4j.NormativeReferenceNode;
import com.linkyoyo.tobacco.entity.neo4j.TocNode;
import com.linkyoyo.tobacco.service.EnhancedStandardDocNeo4jService;
import com.linkyoyo.tobacco.service.GraphVisualizationService;
import com.linkyoyo.tobacco.service.StandardDocService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import javax.validation.constraints.NotNull;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * 知识图谱控制器
 */
@RestController
@RequestMapping("/knowledge")
public class KnowledgeGraphController {

    @Autowired
    private EnhancedStandardDocNeo4jService enhancedStandardDocNeo4jService;

    @Autowired
    private StandardDocService standardDocService;

    @Autowired
    private GraphVisualizationService graphVisualizationService;

    /**
     * 导入标准文档到知识图谱
     * @param standardId 标准文档ID
     * @return 处理结果
     */
    @PostMapping("/standards/{standardId}")
    public ResponseEntity<?> importStandard(@PathVariable Integer standardId) {
        try {
            // 获取标准文档详情
            var detailResponse = standardDocService.getStandardDocWithTocAndContent(standardId);

            if (detailResponse == null) {
                return ResponseEntity.badRequest().body(Map.of(
                    "error", "找不到标准文档: " + standardId
                ));
            }

            // 导入到Neo4j
            EnhancedStandardNode node = enhancedStandardDocNeo4jService.importStandardDocWithToc(
                detailResponse.getStandardDoc(),
                detailResponse.getTocList(),
                detailResponse.getMdContent(),
                detailResponse.getNormativeReferences()
            );

            return ResponseEntity.ok(Map.of(
                "message", "标准文档导入成功",
                "nodeId", node.getId()
            ));
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 批量导入标准文档
     * @param standardIds 标准文档ID列表
     * @return 处理结果
     */
    @PostMapping("/standards/batch")
    public ResponseEntity<?> batchImportStandards(@RequestBody List<Integer> standardIds) {
        try {
            int successCount = enhancedStandardDocNeo4jService.batchImportStandards(standardIds);

            return ResponseEntity.ok(Map.of(
                "message", "批量导入完成",
                "totalCount", standardIds.size(),
                "successCount", successCount
            ));
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取标准文档知识图谱
     * @param standardId 标准文档ID
     * @return 标准文档节点
     */
    @GetMapping("/standards/{standardId}")
    public ResponseEntity<?> getStandardGraph(@PathVariable String standardId) {
        try {
            EnhancedStandardNode node = enhancedStandardDocNeo4jService.findSimplifiedStandardDoc(standardId);
//                    findFullStandardDoc(standardId);

            if (node == null) {
                return ResponseEntity.notFound().build();
            }

            return ResponseEntity.ok(node);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取标准文档的JSON格式简化信息
     * @param standardId 标准文档ID
     * @return 包含属性和关系节点的JSON格式数据
     */
    @GetMapping("/standards/{standardId}/graph")
    public ResponseEntity<?> getStandardDocAsJson(@PathVariable String standardId) {
        try {
            Map<String, Object> result = enhancedStandardDocNeo4jService.findStandardDocAsJson(standardId);

            if (result.isEmpty()) {
                return ResponseEntity.ok(Map.of(
                    "nodes", new java.util.ArrayList<>(),
                    "links", new java.util.ArrayList<>(),
                    "message", "暂无知识图谱数据"
                ));
            }

            return ResponseEntity.ok(result);
        } catch (Exception e) {
            return ResponseEntity.ok(Map.of(
                "nodes", new java.util.ArrayList<>(),
                "links", new java.util.ArrayList<>(),
                "message", "知识图谱服务暂时不可用"
            ));
        }
    }

    /**
     * 根据标准号查询标准文档
     * @param standardNo 标准号
     * @param deptName 部门名称（可选参数）
     * @return 标准文档节点
     */
    @GetMapping("/standards/by-no/{standardNo}")
    public ResponseEntity<?> getStandardByNo(@PathVariable String standardNo,
                                           @RequestParam(required = false) String deptName) {
        try {
            EnhancedStandardNode node;

            // 如果提供了部门名称，使用部门名称和标准号查询
            if (deptName != null && !deptName.trim().isEmpty()) {
                node = enhancedStandardDocNeo4jService.findByStandardNoAndDeptName(standardNo, deptName);
            } else {
                // 否则只使用标准号查询
                node = enhancedStandardDocNeo4jService.findByStandardNo(standardNo);
            }

            if (node == null) {
                return ResponseEntity.notFound().build();
            }

            return ResponseEntity.ok(node);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取标准文档的目录结构
     * @param standardId 标准文档ID
     * @return 目录节点列表
     */
    @GetMapping("/standards/{standardId}/toc")
    public ResponseEntity<?> getTableOfContents(@PathVariable String standardId) {
        try {
            List<TocNode> tocNodes = enhancedStandardDocNeo4jService.findTableOfContents(standardId);

            return ResponseEntity.ok(tocNodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取标准文档的规范性引用文件
     * @param standardId 标准文档ID
     * @return 规范性引用文件节点列表
     */
    @GetMapping("/standards/{standardId}/references")
    public ResponseEntity<?> getNormativeReferences(@PathVariable String standardId) {
        try {
            List<NormativeReferenceNode> referenceNodes = enhancedStandardDocNeo4jService.findNormativeReferences(standardId);

            return ResponseEntity.ok(referenceNodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取标准文档的引用关系（包含引用和被引用）
     * @param standardId 标准文档ID
     * @return 标准引用关系响应
     */
    @GetMapping("/standards/{standardId}/reference-relationships")
    public ResponseEntity<?> getStandardReferenceRelationships(@PathVariable String standardId) {
        try {
            StandardReferenceResponse response = enhancedStandardDocNeo4jService.findStandardReferences(standardId);

            if (response.getStandardNode() == null) {
                return ResponseEntity.notFound().build();
            }

            return ResponseEntity.ok(response);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 查找引用了特定标准的文档
     * @param standardNo 标准号
     * @return 引用该标准的文档节点列表
     */
    @GetMapping("/standards/referencing/{standardNo}")
    public ResponseEntity<?> getReferencingStandards(@PathVariable String standardNo) {
        try {
            List<EnhancedStandardNode> nodes = enhancedStandardDocNeo4jService.findReferencingStandards(standardNo);

            return ResponseEntity.ok(nodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 根据关键字搜索标准文档
     * @param keyword 关键字
     * @return 匹配的标准节点列表
     */
    @GetMapping("/search")
    public ResponseEntity<?> searchStandards(@RequestParam String keyword) {
        try {
            List<EnhancedStandardNode> nodes = enhancedStandardDocNeo4jService.searchByKeyword(keyword);

            return ResponseEntity.ok(nodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 根据规范性引用文件搜索标准文档
     * @param referenceStandardNo 引用的标准号
     * @param referenceType 引用类型
     * @return 匹配的标准节点列表
     */
    @GetMapping("/search-by-reference")
    public ResponseEntity<?> searchByReference(
            @RequestParam(required = false) String referenceStandardNo,
            @RequestParam(required = false) String referenceType) {
        try {
            List<EnhancedStandardNode> nodes = enhancedStandardDocNeo4jService.searchByReference(referenceStandardNo, referenceType);

            return ResponseEntity.ok(nodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取规范性引用文件的统计信息
     * @return 统计信息
     */
    @GetMapping("/reference-statistics")
    public ResponseEntity<?> getReferenceStatistics() {
        try {
            Map<String, Object> statistics = enhancedStandardDocNeo4jService.getReferenceStatistics();

            return ResponseEntity.ok(statistics);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 根据起草单位查询标准文档
     * @param draftUnit 起草单位
     * @return 标准节点列表
     */
    @GetMapping("/standards/by-draft-unit")
    public ResponseEntity<?> getStandardsByDraftUnit(@RequestParam String draftUnit) {
        try {
            List<EnhancedStandardNode> nodes = enhancedStandardDocNeo4jService.findByDraftUnit(draftUnit);

            return ResponseEntity.ok(nodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 根据发布单位查询标准文档
     * @param publishUnit 发布单位
     * @return 标准节点列表
     */
    @GetMapping("/standards/by-publish-unit")
    public ResponseEntity<?> getStandardsByPublishUnit(@RequestParam String publishUnit) {
        try {
            List<EnhancedStandardNode> nodes = enhancedStandardDocNeo4jService.findByPublishUnit(publishUnit);

            return ResponseEntity.ok(nodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 根据标准级别查询标准文档
     * @param standardLevelName 标准级别
     * @return 标准节点列表
     */
    @GetMapping("/standards/by-level")
    public ResponseEntity<?> getStandardsByLevel(@RequestParam String standardLevelName) {
        try {
            List<EnhancedStandardNode> nodes = enhancedStandardDocNeo4jService.findByStandardLevel(standardLevelName);

            return ResponseEntity.ok(nodes);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取标准文档知识图谱可视化数据
     * @param standardId 标准文档ID
     * @param includeContent 是否包含内容
     * @param depth 关系深度
     * @return 图可视化数据
     */
    @GetMapping("/standards/{standardId}/visualization")
    public ResponseEntity<?> getStandardVisualization(
            @PathVariable String standardId,
            @RequestParam(required = false, defaultValue = "false") boolean includeContent,
            @RequestParam(required = false, defaultValue = "1") int depth) {
        try {
            EnhancedStandardNode node = enhancedStandardDocNeo4jService.findFullStandardDoc(standardId);

            if (node == null) {
                return ResponseEntity.notFound().build();
            }

            StandardGraphDto graphDto = graphVisualizationService.convertToGraphDto(node, includeContent, depth);

            return ResponseEntity.ok(graphDto);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }

    /**
     * 获取多个标准文档之间的关系可视化数据
     * @param standardIds 标准文档ID列表
     * @return 图可视化数据
     */
    @PostMapping("/standards/relationship-visualization")
    public ResponseEntity<?> getStandardsRelationship(@RequestBody List<String> standardIds) {
        try {
            if (standardIds == null || standardIds.isEmpty()) {
                return ResponseEntity.badRequest().body(Map.of(
                    "error", "标准文档ID列表不能为空"
                ));
            }

            // 构建完整的图数据
            StandardGraphDto graphDto = new StandardGraphDto();
            List<StandardGraphDto.Node> allNodes = new ArrayList<>();
            List<StandardGraphDto.Link> allLinks = new ArrayList<>();

            // 处理每个标准文档
            for (String standardId : standardIds) {
                EnhancedStandardNode node = enhancedStandardDocNeo4jService.findFullStandardDoc(standardId);
                if (node != null) {
                    StandardGraphDto nodeGraph = graphVisualizationService.convertToGraphDto(node, false, 1);
                    allNodes.addAll(nodeGraph.getNodes());
                    allLinks.addAll(nodeGraph.getLinks());
                }
            }

            // 去重
            Map<String, StandardGraphDto.Node> uniqueNodes = new HashMap<>();
            Map<String, StandardGraphDto.Link> uniqueLinks = new HashMap<>();

            allNodes.forEach(node -> uniqueNodes.put(node.getId(), node));
            allLinks.forEach(link -> uniqueLinks.put(link.getId(), link));

            graphDto.setNodes(new ArrayList<>(uniqueNodes.values()));
            graphDto.setLinks(new ArrayList<>(uniqueLinks.values()));

            return ResponseEntity.ok(graphDto);
        } catch (Exception e) {
            return ResponseEntity.badRequest().body(Map.of(
                "error", e.getMessage()
            ));
        }
    }
}
