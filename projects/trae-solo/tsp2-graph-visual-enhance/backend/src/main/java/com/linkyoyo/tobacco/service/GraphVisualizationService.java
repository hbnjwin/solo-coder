package com.linkyoyo.tobacco.service;

import com.linkyoyo.tobacco.dto.StandardGraphDto;
import com.linkyoyo.tobacco.entity.neo4j.EnhancedStandardNode;
import com.linkyoyo.tobacco.entity.neo4j.NormativeReferenceNode;
import com.linkyoyo.tobacco.entity.neo4j.TocNode;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * 图可视化服务
 * 用于将Neo4j实体转换为前端可视化所需的格式
 */
@Service
public class GraphVisualizationService {

    /**
     * 将标准节点转换为图可视化DTO
     * @param standardNode 标准节点
     * @param includeContent 是否包含内容
     * @param depth 关系深度
     * @return 图可视化DTO
     */
    public StandardGraphDto convertToGraphDto(EnhancedStandardNode standardNode, boolean includeContent, int depth) {
        if (standardNode == null) {
            return new StandardGraphDto();
        }

        StandardGraphDto graphDto = new StandardGraphDto();
        List<StandardGraphDto.Node> nodes = new ArrayList<>();
        List<StandardGraphDto.Link> links = new ArrayList<>();

        // 用于生成唯一ID
        AtomicInteger idGenerator = new AtomicInteger(1);

        // 添加标准节点
        Map<String, Object> standardProperties = new HashMap<>();
        standardProperties.put("standardId", standardNode.getStandardId());

        if (includeContent) {
            standardProperties.put("mdContent", standardNode.getMdContent());
        }

        StandardGraphDto.Node standardNodeDto = StandardGraphDto.Node.builder()
            .id("s" + standardNode.getStandardId())
            .label(standardNode.getStandardName())
            .type("STANDARD")
            .properties(standardProperties)
            .build();

        nodes.add(standardNodeDto);

        // 添加属性节点
        // 标准名称
        if (standardNode.getStandardName() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "standardName", "标准名称", standardNode.getStandardName());
        }

        // 标准英文名
        if (standardNode.getStandardEnName() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "standardEnName", "标准英文名", standardNode.getStandardEnName());
        }

        // 标准体系名称
        if (standardNode.getSystemName() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "systemName", "标准体系名称", standardNode.getSystemName());
        }

        // 标准号
        if (standardNode.getStandardNo() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "standardNo", "标准号", standardNode.getStandardNo());
        }

        // 发布日期
        if (standardNode.getPublishDate() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "publishDate", "发布日期", standardNode.getPublishDate());
        }

        // 标准状态
        if (standardNode.getStatusLabel() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "statusLabel", "标准状态", standardNode.getStatusLabel());
        }

        // 标准级别
        if (standardNode.getStandardLevelName() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "standardLevelName", "标准级别", standardNode.getStandardLevelName());
        }

        // 归属分标委
        if (standardNode.getStandardDeparmentName() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "standardDeparmentName", "归属分标委", standardNode.getStandardDeparmentName());
        }

        // 上传组织
        if (standardNode.getDeptName() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "deptName", "上传组织", standardNode.getDeptName());
        }

        // 发布单位
        if (standardNode.getPublishUnit() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "publishUnit", "发布单位", standardNode.getPublishUnit());
        }

        // 实施日期
        if (standardNode.getImplementDate() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "implementDate", "实施日期", standardNode.getImplementDate());
        }

        // 起草单位
        if (standardNode.getDraftUnit() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "draftUnit", "起草单位", standardNode.getDraftUnit());
        }

        // 起草人
        if (standardNode.getDrafter() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "drafter", "起草人", standardNode.getDrafter());
        }

        // 提出单位
        if (standardNode.getProposeUnit() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "proposeUnit", "提出单位", standardNode.getProposeUnit());
        }

        // 范围
        if (standardNode.getScope() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "scope", "范围", standardNode.getScope());
        }

        // 前言
        if (standardNode.getPreface() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "preface", "前言", standardNode.getPreface());
        }

        // 引言
        if (standardNode.getIntroduction() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "introduction", "引言", standardNode.getIntroduction());
        }

        // 参考文献
        if (standardNode.getReferenceDocs() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "referenceDocs", "参考文献", standardNode.getReferenceDocs());
        }

        // 规范性引用文件
        if (standardNode.getNormativeReferences() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "normativeReferences", "规范性引用文件", standardNode.getNormativeReferences());
        }

        // 术语和定义
        if (standardNode.getTermsAndDefinitions() != null) {
            addPropertyNode(nodes, links, idGenerator, standardNode.getStandardId(),
                "termsAndDefinitions", "术语和定义", standardNode.getTermsAndDefinitions());
        }

        // 添加目录节点
        if (standardNode.getTocEntries() != null) {
            for (TocNode tocNode : standardNode.getTocEntries()) {
                Map<String, Object> tocProperties = new HashMap<>();
                tocProperties.put("sectionNumber", tocNode.getSectionNumber());
                tocProperties.put("sectionTitle", tocNode.getSectionTitle());
                tocProperties.put("pageNumber", tocNode.getPageNumber());
                tocProperties.put("level", tocNode.getLevel());

                String tocId = "t" + idGenerator.getAndIncrement();
                StandardGraphDto.Node tocNodeDto = StandardGraphDto.Node.builder()
                    .id(tocId)
                    .label(tocNode.getSectionTitle())
                    .type("TOC")
                    .properties(tocProperties)
                    .build();

                nodes.add(tocNodeDto);

                // 添加标准到目录的关系
                StandardGraphDto.Link tocLink = StandardGraphDto.Link.builder()
                    .id("l" + idGenerator.getAndIncrement())
                    .source("s" + standardNode.getStandardId())
                    .target(tocId)
                    .type("HAS_TOC")
                    .label("包含章节")
                    .build();

                links.add(tocLink);
            }
        }

        // 添加规范性引用文件节点
        if (standardNode.getNormativeReferenceNodes() != null) {
            for (NormativeReferenceNode referenceNode : standardNode.getNormativeReferenceNodes()) {
                Map<String, Object> refProperties = new HashMap<>();
                refProperties.put("referenceStandardNo", referenceNode.getReferenceStandardNo());
                refProperties.put("referenceStandardName", referenceNode.getReferenceStandardName());
                refProperties.put("referenceType", referenceNode.getReferenceType());
                refProperties.put("referenceStatus", referenceNode.getReferenceStatus());
                refProperties.put("referenceYear", referenceNode.getReferenceYear());

                String refId = "r" + idGenerator.getAndIncrement();
                StandardGraphDto.Node refNodeDto = StandardGraphDto.Node.builder()
                    .id(refId)
                    .label(referenceNode.getReferenceStandardName())
                    .type("REFERENCE")
                    .properties(refProperties)
                    .build();

                nodes.add(refNodeDto);

                // 添加标准到引用的关系
                StandardGraphDto.Link refLink = StandardGraphDto.Link.builder()
                    .id("l" + idGenerator.getAndIncrement())
                    .source("s" + standardNode.getStandardId())
                    .target(refId)
                    .type("NORMATIVELY_REFERENCES")
                    .label("规范引用")
                    .build();

                links.add(refLink);
            }
        }

        // 处理引用关系
        if (depth > 0 && standardNode.getReferences() != null) {
            for (EnhancedStandardNode referencedNode : standardNode.getReferences()) {
                // 递归处理引用的标准，深度减1
                StandardGraphDto referencedGraph = convertToGraphDto(referencedNode, false, depth - 1);

                // 合并节点和关系
                nodes.addAll(referencedGraph.getNodes());
                links.addAll(referencedGraph.getLinks());

                // 添加当前标准到引用标准的关系
                StandardGraphDto.Link referenceLink = StandardGraphDto.Link.builder()
                    .id("l" + idGenerator.getAndIncrement())
                    .source("s" + standardNode.getStandardId())
                    .target("s" + referencedNode.getStandardId())
                    .type("REFERENCES")
                    .label("引用")
                    .build();

                links.add(referenceLink);
            }
        }

        graphDto.setNodes(nodes);
        graphDto.setLinks(links);

        return graphDto;
    }

    /**
     * 添加属性节点
     * @param nodes 节点列表
     * @param links 关系列表
     * @param idGenerator ID生成器
     * @param standardId 标准ID
     * @param propertyKey 属性键
     * @param propertyLabel 属性中文标签
     * @param propertyValue 属性值
     */
    private void addPropertyNode(
            List<StandardGraphDto.Node> nodes,
            List<StandardGraphDto.Link> links,
            AtomicInteger idGenerator,
            String standardId,
            String propertyKey,
            String propertyLabel,
            String propertyValue) {

        if (propertyValue == null || propertyValue.trim().isEmpty()) {
            return;
        }

        // 创建属性节点
        String propId = "p" + idGenerator.getAndIncrement();
        Map<String, Object> propProperties = new HashMap<>();
        propProperties.put("key", propertyKey);
        propProperties.put("value", propertyValue);

        StandardGraphDto.Node propNode = StandardGraphDto.Node.builder()
            .id(propId)
            .label(propertyValue)
            .type("PROPERTY")
            .properties(propProperties)
            .build();

        nodes.add(propNode);

        // 创建关系
        StandardGraphDto.Link propLink = StandardGraphDto.Link.builder()
            .id("l" + idGenerator.getAndIncrement())
            .source("s" + standardId)
            .target(propId)
            .type("HAS_PROPERTY")
            .label(propertyLabel)
            .build();

        links.add(propLink);
    }
}
